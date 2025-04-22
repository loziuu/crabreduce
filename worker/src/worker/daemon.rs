use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

// Daemon workers that runs on machine and executes code from master node
use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};

enum WorkerState {
    IDLE,
    RUNNING,
    COMPLETED,
}

pub struct DaemonWorker {
    state: WorkerState,
    curr_threads: usize,
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

impl DaemonWorker {
    pub fn new(config: WorkerConfiguration) -> DaemonWorker {
        Self {
            curr_threads: 0,
            state: WorkerState::IDLE,
            config,
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
