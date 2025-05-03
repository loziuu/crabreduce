use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
    node_id::NodeId,
    worker::WorkerState,
};
use gethostname::gethostname;

use crate::rpc::{HeartbeatRequest, Id, RegisterRequest};

use super::{Worker, WorkerError, master_client::MasterClient};

/// Uni Worker is
pub struct UniWorker<J: Job> {
    state: WorkerState,
    curr_threads: usize,
    job: J,
    config: WorkerConfiguration,
    client: MasterClient,
    is_registered: bool,
}

pub struct WorkerConfiguration {
    id: NodeId,
    max_threads: usize,
    server: SocketAddr,
    job_type: String,
}

impl Default for WorkerConfiguration {
    fn default() -> Self {
        Self {
            max_threads: 1,
            id: NodeId::raw(gethostname().to_str().unwrap().to_string()),
            server: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 50420)),
            job_type: "Default".to_string(),
        }
    }
}

impl<J: Job> UniWorker<J> {
    pub fn new(config: WorkerConfiguration, job: J, rpc_client: MasterClient) -> UniWorker<J> {
        Self {
            curr_threads: 0,
            state: WorkerState::IDLE,
            config,
            job,
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
        let values = task.reduce(k, value);
        //persist(values);
    }
}

impl<J: Job> Worker for UniWorker<J> {
    async fn register(&mut self) -> Result<(), WorkerError> {
        let req = RegisterRequest {
            worker_id: Some(Id {
                id: gethostname().to_str().unwrap().to_string(),
            }),
        };

        // TODO: Add adding name from config
        let _ = self.client.register(req).await;
        self.is_registered = true;
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

pub enum WorkerSignal {
    RunJob,
    Heartbeat,
    Shutdown,
}
