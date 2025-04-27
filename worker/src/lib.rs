pub mod rpc {
    tonic::include_proto!("proto/crab_master_service.proto");
}

pub mod worker;
