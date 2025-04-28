use crate::server::crab_master_service_server::CrabMasterServiceServer;
use node::master::MasterNode;
use std::error::Error;
use tonic::transport::Server;
use tracing::info;

mod node;

pub mod server {
    tonic::include_proto!("crabmaster");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get port from args
    config_tracing();

    let addr = "[::1]:50420".parse()?;
    let master = MasterNode::new();

    info!("Starting CrabReduce server @ {:?}", addr);
    Server::builder()
        .add_service(CrabMasterServiceServer::new(master))
        .serve(addr)
        .await?;

    Ok(())
}

fn config_tracing() {
    tracing_subscriber::fmt()
        // Only during development?
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .with_thread_ids(true)
        .init();
}
