use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};

use common::types::node_id::NodeId;
use tokio::{sync::Mutex, time::sleep};
use tracing::info;

use super::worker::Worker;

pub struct MasterJanitor {
    master: Arc<Mutex<HashMap<NodeId, Worker>>>,
    timeout: Duration,
}

impl MasterJanitor {
    pub fn new(master: Arc<Mutex<HashMap<NodeId, Worker>>>, timeout: Duration) -> Self {
        Self { master, timeout }
    }

    // NOTE: RwLock may be better in my opinion...
    pub fn run(self) {
        let workers = self.master.clone();

        // NOTE: Use ticker instead of sleep?
        tokio::spawn(async move {
            loop {
                {
                    info!("Checking workers liveness.");
                    let now = SystemTime::now();
                    let mut m = workers.lock().await;
                    info!("There are {} workers.", m.len());

                    let to_remove: Vec<NodeId> = m
                        .iter()
                        .filter(|(_, worker)| {
                            now.duration_since(*worker.last_heartbeat()).unwrap() > self.timeout
                        })
                        .map(|(id, _)| id)
                        .cloned()
                        .collect();

                    for node in to_remove {
                        info!("Evicting {}.", node);
                        m.remove_entry(&node);
                    }
                }

                sleep(self.timeout).await;
            }
        });
    }
}
