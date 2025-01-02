use axum::{extract::Query, response::Redirect};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::OIDC_TAG;

/// Authorization endpoint.
///
/// Allows the client to request authorization from the OIDC provider.
#[utoipa::path(
        get,
        path = "/authorize",
        tag = OIDC_TAG,
        responses(
            (status = 302, description = "Successful authorization request")
        )
    )]
pub async fn get_authorize(_query: Query<AuthorizationRequestQuery>) -> Redirect {
    Redirect::temporary("https://example.com")
}

#[derive(Deserialize, ToSchema)]
pub struct AuthorizationRequestQuery {
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: String,
}

// exmaple GET call: http://localhost/authorize?client_id=test&response_type=code&redirect_uri=http://example.com&scope=openid&state=123
