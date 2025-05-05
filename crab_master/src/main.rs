use crate::node::janitor::MasterJanitor;
use crate::server::crab_master_service_server::CrabMasterServiceServer;
use node::master::MasterNode;
use std::error::Error;
use std::time::Duration;
use tonic::transport::Server;
use tracing::info;

mod node;

pub mod server {
    tonic::include_proto!("crabmaster");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    common::tracing::init();

    let addr = "[::1]:50420".parse()?;

    let master = MasterNode::new();
    MasterJanitor::new(master.borrow_workers(), Duration::from_secs(60)).run();

    info!("Starting CrabReduce server @ {:?}", addr);
    Server::builder()
        .add_service(CrabMasterServiceServer::new(master))
        .serve(addr)
        .await?;

    Ok(())
}
