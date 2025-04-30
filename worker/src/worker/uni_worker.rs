use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
    node_id::NodeId,
    worker::WorkerState,
};
use gethostname::gethostname;
use tokio::sync::mpsc;
use tracing::info;

use crate::rpc::{Id, RegisterRequest};

use super::{Worker, heartbeat::HeartbeatManager, master_client::MasterClient};

/// Uni Worker is
pub struct UniWorker<J: Job> {
    state: WorkerState,
    curr_threads: usize,
    job: J,
    config: WorkerConfiguration,
    client: MasterClient,
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
    async fn shutdown(&mut self) {
        panic!("Just panic");
    }

    async fn register(&mut self) {
        let req = RegisterRequest {
            worker_id: Some(Id {
                id: gethostname().to_str().unwrap().to_string(),
            }),
        };

        // TODO: Add adding name from config
        let _ = self.client.register(req).await;

        // TODO: Setup channels to get tasks and shutdowns?
        // TODO: Config buffer size?
        let (tx, mut rx) = mpsc::channel::<WorkerSignal>(32);

        let heartbeat = HeartbeatManager::new(self.config.id.clone(), self.client.clone());
        heartbeat.start(tx.clone());

        while let Some(msg) = rx.recv().await {
            match msg {
                WorkerSignal::RunJob => {
                    info!("Received run job signal!");
                }
                WorkerSignal::Heartbeat => {
                    info!("Received heartbeat!");
                }
                WorkerSignal::Shutdown => {
                    info!("Received shutdown!");
                }
            }
        }
    }
}

pub enum WorkerSignal {
    RunJob,
    Heartbeat,
    Shutdown,
}
