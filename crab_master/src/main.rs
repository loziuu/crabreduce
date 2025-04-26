use crate::server::crab_master_service_server::CrabMasterServiceServer;
use node::master::MasterNode;
use std::error::Error;
use tonic::transport::Server;

mod node;

pub mod server {
    tonic::include_proto!("crabmaster");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get port from args
    let addr = "[::1]:50420".parse()?;
    let master = MasterNode::new();

    println!("Starting CrabReduce server @ {:?}", addr);
    Server::builder()
        .add_service(CrabMasterServiceServer::new(master))
        .serve(addr)
        .await?;

    Ok(())
}
