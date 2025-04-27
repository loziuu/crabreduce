pub mod worker;

pub mod rpc {
    tonic::include_proto!("crabmaster");
}
