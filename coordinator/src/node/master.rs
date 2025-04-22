use tonic::Response;

use crate::server::EchoRequest;
use crate::server::EchoResponse;
use crate::server::echo_server::Echo;

pub struct MasterConfiguration {}

pub struct MasterNode {}

impl Default for MasterNode {
    fn default() -> Self {
        MasterNode::new()
    }
}

impl MasterNode {
    pub fn new() -> MasterNode {
        MasterNode {}
    }
}

#[tonic::async_trait]
impl Echo for MasterNode {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> std::result::Result<tonic::Response<EchoResponse>, tonic::Status> {
        let body = request.get_ref();

        let reply = EchoResponse {
            msg: body.msg.clone(),
        };

        Ok(Response::new(reply))
    }
}
