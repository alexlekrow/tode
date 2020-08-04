use std::env;

use tonic::{transport::Server, Request, Response, Status};

use tode::tode_server::{Tode, TodeServer};
use tode::{GetHealthRequest, GetHealthResponse};

pub mod tode {
    tonic::include_proto!("tode");
}

#[derive(Debug, Default)]
pub struct TonicNode {}

#[tonic::async_trait]
impl Tode for TonicNode {
    async fn get_health(
        &self,
        request: Request<GetHealthRequest>,
    ) -> Result<Response<GetHealthResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tode::GetHealthResponse {
            is_healthy: true,
            conditions: vec![
                tode::Condition{
                    info: "Entered Safe condition".to_string(),
                },
                tode::Condition{
                    info: "Entered Unsafe condition".to_string(),
                }
            ]
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Running this inside a container requires we serve on 0.0.0.0 not localhost
    let grpc_addr = "[::0]:".to_string() + &env::var("TODE_PORT").unwrap_or("50051".to_string());
    let tonic_node = TonicNode::default();
    
    println!("Serving Tonic gRPC server on {}", grpc_addr);
    Server::builder()
        .add_service(TodeServer::new(tonic_node))
        .serve(grpc_addr.parse()?)
        .await?;
    println!("Exiting Tonic gRPC server on {}", grpc_addr);
    
    Ok(())
}
