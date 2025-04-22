use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};

pub mod daemon;
pub mod uni_worker;

enum WorkerState {
    IDLE,
    RUNNING,
    COMPLETED,
}

pub struct Worker {
    id: usize,

    state: WorkerState,

    curr_threads: usize,
    max_threads: usize,
}

pub struct WorkerConfiguration {
    // RPC Connection info
    max_threads: usize,
    id: Option<usize>,
}

impl Worker {
    pub fn new(config: WorkerConfiguration) -> Worker {
        Self {
            id: config.id.unwrap_or(1),
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
