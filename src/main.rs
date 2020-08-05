use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod tode {
    tonic::include_proto!("tode");
}

use tode::{
    tode_server::{Tode, TodeServer},
    GetHealthRequest, GetHealthResponse,
};

#[derive(Debug, Default)]
pub struct TonicNode {}

#[tonic::async_trait]
impl Tode for TonicNode {

    #[tracing::instrument]
    async fn get_health(
        &self,
        request: Request<GetHealthRequest>,
    ) -> Result<Response<GetHealthResponse>, Status> {
        // tracing::info!("GetHealth: request={:?}", request);

        let response = Response::new(tode::GetHealthResponse {
            is_healthy: true,
            conditions: vec![
                tode::Condition {
                    info: "Entered Safe condition".to_string(),
                },
                tode::Condition {
                    info: "Entered Unsafe condition".to_string(),
                },
            ],
        });
        tracing::info!("GetHealth: response={:?}", response);
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // NOTE: Running this inside a container requires we serve on 0.0.0.0 not localhost
    let addr = "[::0]:".to_string() + &env::var("TODE_PORT").unwrap_or("50051".to_string());
    let tode = TonicNode::default();

    tracing::info!(message ="Starting gRPC server.", %addr);
    Server::builder()
        .trace_fn(|_| tracing::info_span!("tode_server"))
        .add_service(TodeServer::new(tode))
        .serve(addr.parse()?)
        .await?;
    tracing::info!(message ="Exiting gRPC server.", %addr);
    Ok(())
}
