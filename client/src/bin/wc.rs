use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};
use worker::worker::{
    master_client::RpcCrabMaster,
    uni_worker::{Config, UniWorker},
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

    let master = RpcCrabMaster::new("http://[::1]:50420".to_string());

    // NOTE: Don't create worker here, move it to worker::some_fancy_new_method(cfg, job, client);
    let v = UniWorker::new(Config::default(), WordCount {}, master);

    worker::start_worker(v).await;

    Ok(())
}
