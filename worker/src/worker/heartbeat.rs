use std::{sync::atomic::AtomicI8, time::Instant};

use common::types::node_id::NodeId;
use tokio::sync::mpsc::Sender;

use crate::rpc::HeartbeatRequest;

use super::{master_client::MasterClient, uni_worker::WorkerSignal};

// TODO: Move it to config struct?
const HEARTBEAT_INTERVAL: u64 = 5;
const SHUTDOWN_THRESHOLD: usize = 10;

pub struct HeartbeatManager {
    id: NodeId,
    client: MasterClient,
    last_time: Instant,
}

impl HeartbeatManager {
    pub fn new(id: NodeId, client: MasterClient) -> Self {
        Self {
            id,
            client,
            last_time: Instant::now(),
        }
    }

    pub async fn send_heartbeat(&mut self) {
        self.last_time = Instant::now();
    }

    /// Start firing heartbeats and publish an shutdown signal when it fails N times?
    // TODO: Abstract away sender so it's not depending on tokio API that much...
    pub fn start(self, sender: Sender<WorkerSignal>) {
        let mut retries = 0;
        tokio::spawn(async move {
            if retries >= 10 {
                sender.send(WorkerSignal::Shutdown).await;
                return;
            }

            let req = HeartbeatRequest {};

            match self.client.heartbeat(req).await {}
        });
    }
}
