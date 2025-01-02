use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddr};
use utoipa::OpenApi;

use tokio::net::TcpListener;

use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

mod oidc;

const OIDC_TAG: &str = "OIDC";

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = OIDC_TAG, description = "OIDC implementation."),
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/", oidc::router())
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/scalar", api));

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 80));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
