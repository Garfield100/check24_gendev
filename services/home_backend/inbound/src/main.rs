mod app_error;

mod routes;

use config::Config;
use domain::HomeService;
use domain::WidgetRepository;
use outbound::WidgetCache;
use std::{path::Path, sync::Arc};
use tower_http::services::ServeDir;
use tracing::Level;

use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

const DIST_PATH: &str = "../../../clients/web/out/";

#[derive(Debug, Clone)]
struct AppState {
    home_service: Arc<HomeService<WidgetCache>>,
}

#[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_backtrace::install();

    // logging to terminal
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

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
        .routes(routes!(routes::get_recommendations))
        .routes(routes!(routes::set_recommendation))
        .split_for_parts();

    let mut home_service = HomeService {
        widget_cache: WidgetCache::new().await?,
    };
    home_service.widget_cache.clear().await?;

    let app_state = AppState {
        home_service: Arc::new(home_service),
    };

    let app = app
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .with_state(app_state);

    let local_bind: String = settings.get("local_bind")?;
    let listener = tokio::net::TcpListener::bind(&local_bind).await?;
    info!("Starting server on http://{local_bind}");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
