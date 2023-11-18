use chrono::Utc;
use lazy_static::lazy_static;
use log::{LevelFilter, Record};
use serde_json::{Map, Value};
use std::{collections::HashMap, env, sync::RwLock};

lazy_static! {
    static ref PERSISTENT_LOGS: RwLock<HashMap<String, serde_json::Value>> =
        RwLock::new(HashMap::new());
}

pub fn add_persistent_log(key: &str, value: Value) {
    let mut logs = PERSISTENT_LOGS.write().unwrap();
    logs.insert(key.to_string(), value);
}

pub fn get_persistent_log(key: &str) -> Option<Value> {
    let logs = PERSISTENT_LOGS.read().unwrap();
    logs.get(key).cloned()
}

pub fn get_all_persistent_logs() -> HashMap<String, Value> {
    let logs = PERSISTENT_LOGS.read().unwrap();
    logs.clone()
}

pub fn clear_persistent_logs() {
    *PERSISTENT_LOGS.write().unwrap() = HashMap::new();
}

#[macro_export]
macro_rules! add_persistent_logs {
    ($($key:expr => $value:expr),*) => {{
        $(
            structured_persistent_logger::add_persistent_log($key, serde_json::to_value($value).unwrap());
        )*
    }};
}

pub struct StructuredPersistentLogger {}

impl StructuredPersistentLogger {
    pub fn new() -> Self {
        StructuredPersistentLogger {}
    }

    pub fn init() {
        let log_level = match env::var("RUST_LOG") {
            Ok(value) => match value.to_lowercase().as_str() {
                "error" => LevelFilter::Error,
                "warn" => LevelFilter::Warn,
                "info" => LevelFilter::Info,
                "debug" => LevelFilter::Debug,
                "trace" => LevelFilter::Trace,
                _ => LevelFilter::Info,
            },
            Err(_) => LevelFilter::Off,
        };
        log::set_max_level(log_level);
        log::set_boxed_logger(Box::new(StructuredPersistentLogger::new())).unwrap();
    }
}

impl log::Log for StructuredPersistentLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        let mut field_map = Map::new();
        for (k, v) in get_all_persistent_logs().into_iter() {
            field_map.insert(k, v);
        }

        field_map.insert("level".into(), record.level().to_string().into());
        field_map.insert("message".into(), record.args().to_string().into());

        let timestamp = Utc::now().to_rfc3339();
        field_map.insert("timestamp".to_string(), Value::String(timestamp));

        let record = serde_json::Value::Object(field_map);
        if let Ok(json) = serde_json::to_string(&record) {
            println!("{}", json)
        }
    }

    fn flush(&self) {}
}
