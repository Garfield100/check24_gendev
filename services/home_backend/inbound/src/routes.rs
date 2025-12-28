use crate::app_error::AppError;

use super::AppState;
use domain::Personalisation;
use domain::UserID;
use domain::WidgetRepository;

use axum::extract;
use axum::extract::State;
use axum::response;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
pub(crate) struct Recommendations {
    pub(crate) user_id: Uuid,
    pub(crate) recs_by_product: HashMap<String, serde_json::Value>,
}

// In a real environment this would be authenticated and authorised

#[tracing::instrument]
#[axum::debug_handler] // no effect in release profile
#[utoipa::path(get,
    path = "/get_recommendations/{user_id}",
    responses(
        (status = OK, body=Recommendations)
    ),
)]
pub(crate) async fn get_recommendations(
    State(state): State<AppState>,
    extract::Path(user_id): extract::Path<Uuid>,
) -> response::Result<axum::Json<Recommendations>> {
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

// here we return a String for legibility but in a real prod system I would send bytes to halve the size

#[tracing::instrument]
#[axum::debug_handler]
#[utoipa::path(get, path = "/get_cached_users")]
pub(crate) async fn get_cached_users(State(state): State<AppState>) -> response::Result<String> {
    Ok(state
        .home_service
        .widget_cache
        .get_cached_users()
        .await
        .map_err(AppError)?
        .iter()
        .map(|id| id.0.to_string() + "\n")
        .collect())
}
