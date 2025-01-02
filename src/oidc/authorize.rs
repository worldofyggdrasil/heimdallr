use axum::{response::Redirect, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::OIDC_TAG;

/// Authorization endpoint.
///
/// Allows the client to request authorization from the OIDC provider.
#[utoipa::path(
        get,
        path = "/authroize",
        tag = OIDC_TAG,
        responses(
            (status = 302, description = "Successful authorization request")
        )
    )]
pub async fn get_authorize(_request: Json<AuthorizationRequest>) -> Redirect {
    Redirect::temporary("https://example.com")
}

#[derive(Deserialize, ToSchema)]
pub struct AuthorizationRequest {
    pub _client_id: String,
    pub _response_type: String,
    pub _redirect_uri: String,
    pub _scope: String,
    pub _state: String,
}
