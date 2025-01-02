use utoipa_axum::{router::OpenApiRouter, routes};

pub mod authorize;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(authorize::get_authorize))
}
