use common::types::{
    job::Job,
    kv::{Key, KeyValue, Value},
};
use worker::worker::uni_worker::{UniWorker, WorkerConfiguration};

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

fn main() {
    let job = WordCount {};
    let v = UniWorker::new(WorkerConfiguration::default(), job);
    v.register();
}
