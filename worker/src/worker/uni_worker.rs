use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};

enum WorkerState {
    IDLE,
    RUNNING,
    COMPLETED,
}

pub struct UniWorker<J: Job> {
    state: WorkerState,
    curr_threads: usize,
    job: J,
    config: WorkerConfiguration,
}

pub struct WorkerConfiguration {
    // RPC Connection info
    id: usize,
    max_threads: usize,
    server: SocketAddr,
}

impl Default for WorkerConfiguration {
    fn default() -> Self {
        Self {
            max_threads: 1,
            id: 1,
            server: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 50420)),
        }
    }
}

impl<J: Job> UniWorker<J> {
    pub fn new(config: WorkerConfiguration, job: J) -> UniWorker<J> {
        Self {
            curr_threads: 0,
            state: WorkerState::IDLE,
            config,
            job,
        }
    }

    pub fn register(&self) {
        // Connect to coordinator
        // Run loop
        // Handle shutdown
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
