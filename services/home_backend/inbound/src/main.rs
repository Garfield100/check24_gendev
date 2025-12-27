mod app_error;

use std::{collections::HashMap, path::Path, sync::Arc};

use axum::{
    extract::{self, State},
    response,
};
use config::Config;
use domain::WidgetRepository;
use domain::{HomeService, Personalisation, UserID};
use outbound::WidgetCache;
use serde_json::{Value, json};
use tower_http::services::ServeDir;

use tracing::{info, warn};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::app_error::AppError;

const DIST_PATH: &str = "../../../clients/web/out/";

#[derive(Debug, Clone)]
struct AppState {
    home_service: Arc<HomeService<WidgetCache>>,
}

// #[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_backtrace::install();

    let settings = Config::builder()
        .add_source(config::File::with_name("../../../settings.toml"))
        .add_source(config::File::with_name("settings.toml").required(false))
        .add_source(config::Environment::with_prefix("GENDEV"))
        .build()?;

    let dist_path = Path::new(DIST_PATH);

    assert!(dist_path.exists());
    assert!(dist_path.is_dir());

    let (app, api) = OpenApiRouter::new()
        .fallback_service(ServeDir::new(dist_path))
        .routes(routes!(get_recommendations))
        .split_for_parts();

    let app_state = AppState {
        home_service: Arc::new(HomeService {
            widget_cache: WidgetCache::new().await?,
        }),
    };

    let app = app
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .with_state(app_state);

    let local_bind: String = settings.get("local_bind")?;
    let listener = tokio::net::TcpListener::bind(&local_bind).await?;
    info!("Starting server on http://{local_bind}");
    axum::serve(listener, app).await.unwrap();

    // tokio::

    Ok(())
}

// #[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
// struct UserID(pub Uuid);

#[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
struct Recommendations {
    user_id: Uuid,
    recs_by_product: HashMap<String, serde_json::Value>,
}

// In a real environment this would be authenticated and authorised
#[axum::debug_handler] // no effect in release profile
#[utoipa::path(get,
    path = "/get_recommendations/{user_id}",
    responses(
        (status = OK, body=Recommendations)
    ),
    // params(
    //     ("user_id" = Uuid, Path, description = "Get recommendations for user with the given UUID. If no user-specific rec is found, the generic one is returned immediately while a new recommendation is created for later.")
    // )
)]
async fn get_recommendations(
    State(state): State<AppState>,
    extract::Path(user_id): extract::Path<Uuid>,
) -> response::Result<axum::Json<Recommendations>> {
    // let mut foo_recs = HashMap::new();
    // foo_recs.insert("foo".into(), json!({"foo": "bar"}));

    let personalisation = Personalisation(Some(UserID(user_id)));
    let recs = state
        .home_service
        .widget_cache
        .get_widgets_for_user(&personalisation)
        .await
        .map_err(AppError)?;

    let recs = recs
        .into_iter()
        .map(|w| (String::from(w.product), Value::from(w.data)))
        .collect();

    Ok(axum::Json::from(Recommendations {
        user_id,
        recs_by_product: recs,
    }))
}
