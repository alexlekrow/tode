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

// TODO: Make PORT an ENV variable
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Running this inside a container requires we serve on 0.0.0.0 not localhost
    let default_port = "50051".to_string();
    let server_port = env::var("TODE_PORT").unwrap_or(default_port);
    let address = "[::0]:".to_string() + &server_port;
    let tonic_node = TonicNode::default();
    
    println!("Serving Tonic gRPC server on {}", address);
    Server::builder()
        .add_service(TodeServer::new(tonic_node))
        .serve(address.parse()?)
        .await?;
    println!("Exiting Tonic gRPC server on {}", address);
    
    Ok(())
}
