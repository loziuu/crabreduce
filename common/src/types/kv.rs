use std::ops::Deref;

pub struct Key(String);
pub struct Value(String);

pub struct KeyValue {
    key: Key,
    value: Value,
}

impl KeyValue {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Deref for Value {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value(value)
    }
}

impl From<(Key, Value)> for KeyValue {
    fn from(value: (Key, Value)) -> Self {
        KeyValue {
            key: value.0,
            value: value.1,
        }
    }
}
