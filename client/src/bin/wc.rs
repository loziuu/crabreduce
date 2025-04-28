use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};
use tracing::info;
use worker::worker::{
    master_client::MasterClient,
    uni_worker::{UniWorker, WorkerConfiguration},
};

struct WordCount {}

impl Job for WordCount {
    fn map(&self, kv: KeyValue) -> Vec<KeyValue> {
        let val = kv.value();

        val.split_whitespace()
            .map(|it| KeyValue::new(it.to_string(), format!("{}", 1)))
            .collect()
    }

    fn reduce(&self, k: Key, v: Vec<Value>) -> KeyValue {
        let mut sum = 0;

        for cnt in v {
            sum += cnt.parse::<i32>().unwrap();
        }

        (k, Value::from(format!("{}", sum))).into()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    common::tracing::init();
    info!("Starting CrabReduce worker...");

    let job = WordCount {};

    let addr = "http://[::1]:50420";

    info!("Trying to regsiter to master: [{}]", addr);
    let connect = MasterClient::connect(addr).await?;
    let mut v = UniWorker::new(WorkerConfiguration::default(), job, connect);
    v.register().await;
    info!("Worker started successfully");
    Ok(())
}
