use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
    node_id::NodeId,
    worker::WorkerState,
};
use gethostname::gethostname;

use crate::rpc::{HeartbeatRequest, Id, RegisterRequest};

use super::{Worker, WorkerError, master_client::CrabMaster};

/// Uni Worker is a type of worker that does just one concrete job.
pub struct UniWorker<J: Job, C: CrabMaster> {
    state: WorkerState,
    config: WorkerConfiguration,
    client: C,
    is_registered: bool,

    _curr_threads: usize,
    _job: J,
}

pub struct WorkerConfiguration {
    id: NodeId,

    _max_threads: usize,
    _server: SocketAddr,
    _job_type: String,
}

impl Default for WorkerConfiguration {
    fn default() -> Self {
        Self {
            id: NodeId::raw(gethostname().to_str().unwrap().to_string()),

            _max_threads: 1,
            _server: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 50420)),
            _job_type: "Default".to_string(),
        }
    }
}

impl<J: Job, MC: CrabMaster> UniWorker<J, MC> {
    pub fn new(config: WorkerConfiguration, job: J, rpc_client: MC) -> UniWorker<J, MC> {
        Self {
            _curr_threads: 0,
            state: WorkerState::Detached,
            config,
            _job: job,
            client: rpc_client,
            is_registered: false,
        }
    }

    pub fn map(task: &impl Job, kv: KeyValue) {
        // Load file from task
        task.map(kv);
        // Persist to local disk
    }

    pub fn reduce(task: &impl Job, k: Key, value: Vec<Value>) {
        // Get from local disk and reduce and save to output file
        let _values = task.reduce(k, value);
        //persist(values);
    }
}

impl<J: Job, MC: CrabMaster> Worker for UniWorker<J, MC> {
    async fn register(&mut self) -> Result<(), WorkerError> {
        if self.is_registered {
            return Ok(());
        }

        self.client
            .connect()
            .await
            .map_err(|_| WorkerError::ConnectionError)?;

        let req = RegisterRequest {
            worker_id: Some(Id {
                id: self.config.id.to_string(),
            }),
        };

        // TODO: Add adding name from config
        let _ = self.client.register(req).await;
        self.is_registered = true;
        self.state = WorkerState::Idle;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), super::WorkerError> {
        panic!("Just panic...");
    }

    async fn heartbeat(&mut self) -> Result<(), WorkerError> {
        if !self.is_registered {
            return Err(WorkerError::NotRegistered);
        }

        let req = HeartbeatRequest {
            id: Some(Id {
                id: self.config.id.to_string(),
            }),
            state: 0,
        };

        if self.client.heartbeat(req).await.is_err() {
            return Err(WorkerError::ConnectionError);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    struct MockJob {}

    impl Job for MockJob {
        fn map(&self, _kv: KeyValue) -> Vec<KeyValue> {
            vec![]
        }

        fn reduce(&self, _k: Key, _v: Vec<Value>) -> KeyValue {
            KeyValue::new("test".to_string(), "mock".to_string())
        }
    }

    use crate::worker::master_client::{MockCrabMaster, RpcError};

    use super::*;

    #[test]
    fn init_new_worker() {
        let worker = UniWorker::new(test_config(), MockJob {}, setup_mock(MockCrabMaster::new()));

        assert_eq!(worker.state, WorkerState::Detached);
    }

    #[test]
    fn register_worker() {
        let mut worker =
            UniWorker::new(test_config(), MockJob {}, setup_mock(MockCrabMaster::new()));

        futures::executor::block_on(worker.register()).unwrap();

        assert_eq!(worker.state, WorkerState::Idle);
    }

    #[test]
    fn fail_connect_during_register() {
        let mut master_mock = MockCrabMaster::new();
        master_mock
            .expect_connect()
            .returning(|| Box::pin(async { Err(RpcError::CannotConnect) }));
        let mut worker = UniWorker::new(test_config(), MockJob {}, setup_mock(master_mock));

        let _ = futures::executor::block_on(worker.register());

        assert_eq!(worker.state, WorkerState::Detached);
    }

    fn setup_mock(mut master: MockCrabMaster) -> MockCrabMaster {
        master
            .expect_register()
            .returning(|_| Box::pin(async { Ok(()) }));

        master
            .expect_connect()
            .returning(|| Box::pin(async { Ok(()) }));

        master
            .expect_heartbeat()
            .returning(|_| Box::pin(async { Ok(()) }));

        master
    }

    fn test_config() -> WorkerConfiguration {
        WorkerConfiguration::default()
    }
}
