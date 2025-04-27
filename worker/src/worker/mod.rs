use common::types::{
    client_id::NodeId,
    job::Job,
    kv::{Key, KeyValue, Value},
    worker::WorkerState,
};

pub mod daemon;
pub mod rpc_client;
pub mod uni_worker;

pub struct Worker {
    id: NodeId,

    state: WorkerState,

    curr_threads: usize,
    max_threads: usize,
}

pub struct WorkerConfiguration {
    // RPC Connection info
    max_threads: usize,
    id: NodeId,
}

impl Worker {
    pub fn new(config: WorkerConfiguration) -> Worker {
        Self {
            id: config.id,
            curr_threads: 0,
            max_threads: config.max_threads,
            state: WorkerState::IDLE,
        }
    }

    fn connect(&mut self) {
        // Connect to coordinator
    }

    pub fn map(task: &impl Job, kv: KeyValue) {
        task.map(kv);
        // Persist to local disk
    }

    pub fn reduce(task: &impl Job, k: Key, value: Vec<Value>) {
        // Get from local disk and reduce and save to output file
        let values = task.reduce(k, value);
        //persist(values);
    }
}
