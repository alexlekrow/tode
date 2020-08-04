use std::env;

use tode::tode_client::TodeClient;
use tode::GetHealthRequest;

pub mod tode {
    tonic::include_proto!("tode");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_port = "50051".to_string();
    let server_port = env::var("TODE_PORT").unwrap_or(default_port);
    let address = "http://[::1]:".to_string() + &server_port;

    println!("Client connection to Tonic gRPC server on {:?}", address);
    let mut client = TodeClient::connect(address).await?;

    let request = tonic::Request::new(GetHealthRequest {});
    let response = client.get_health(request).await?;
    println!("Client got Response: {:?}", response);

    Ok(())
}