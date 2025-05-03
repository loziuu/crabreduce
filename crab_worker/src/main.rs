pub mod rpc {
    tonic::include_proto!("crabmaster");
}

fn main() {
    common::tracing::init();
}
