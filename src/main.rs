use warp::{Filter, Rejection, Reply};
  
// use std::collections::HashMap;
// use std::hash::{Hash, Hasher};
// use std::pin::Pin;
// use std::sync::Arc;
// use std::time::Instant;

// use futures::{Stream, StreamExt};

// use tokio::sync::mpsc;

// use tonic::transport::Server;
// use tonic::{Request, Response, Status};

// use tode::tode_server::{Tode, TodeServer};
// use tode::{Condition, GetHealthRequest, GetHealthResponse};

// pub mod tode {
//     tonic::include_proto!("tode");
// }

// #[derive(Debug)]
// pub struct TodeService {
//     is_healthy: bool,
//     features: Arc<Vec<Condition>>,
// }

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").and_then(health_handler);

    let routes = health_route.with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}