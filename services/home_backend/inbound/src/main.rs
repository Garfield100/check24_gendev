use std::{collections::HashMap, path::Path};

use axum::{extract, response};
use config::Config;
use serde_json::json;
use tower_http::services::ServeDir;

use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

const DIST_PATH: &str = "../../../clients/web/out/";

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

    let app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    let local_bind: String = settings.get("local_bind")?;
    let listener = tokio::net::TcpListener::bind(&local_bind).await?;
    println!("Starting server on http://{local_bind}");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
struct UserID(pub Uuid);

#[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
struct Recommendations {
    user_id: UserID,
    recs_by_product: HashMap<String, serde_json::Value>,
}

// In a real environment this would be authenticated and authorised
#[axum::debug_handler]
#[utoipa::path(get, path = "/get_recommendations", responses((status = OK, body=Recommendations)))]
async fn get_recommendations(
    extract::Path(user_id): extract::Path<UserID>,
) -> response::Result<axum::Json<Recommendations>> {
    let mut foo_recs = HashMap::new();
    foo_recs.insert("foo".into(), json!({"foo": "bar"}));

    Ok(axum::Json::from(Recommendations {
        user_id,
        recs_by_product: foo_recs,
    }))
}
