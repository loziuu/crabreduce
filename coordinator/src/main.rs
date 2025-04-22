use node::master::MasterNode;
use server::echo_server::EchoServer;
use std::error::Error;
use tonic::transport::Server;

mod node;

pub mod server {
    tonic::include_proto!("coordinator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get port from args
    let addr = "[::1]:50420".parse()?;
    let master = MasterNode::new();

    println!("Starting CrabReduce server @ {:?}", addr);
    Server::builder()
        .add_service(EchoServer::new(master))
        .serve(addr)
        .await?;

    Ok(())
}
