use std::sync::atomic::{AtomicI64, Ordering};
use chrono::{DateTime, TimeZone, Utc};

pub const EPOCH: i64 = 1420070400000;
const TIMESTAMP_OFFSET: i64 = 22;
const WORKER_ID_OFFSET: i64 = 17;
const DATACENTER_ID_OFFSET: i64 = 12;
const MAX_SEQUENCE: i64 = 1 << 12;

pub struct SnowflakeGenerator {
    pub worker_id: i64,
    pub datacenter_id: i64,
    sequence: AtomicI64,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: i64, datacenter_id: i64) -> Self {
        Self {
            worker_id,
            datacenter_id,
            sequence: AtomicI64::new(0),
        }
    }

    pub fn generate(&self) -> Snowflake {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let sequence = self.sequence.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |v| Some(if v < MAX_SEQUENCE { v + 1 } else { 0 }),
        ).unwrap();

        let epoch_offset = timestamp - EPOCH;
        let value = (epoch_offset << TIMESTAMP_OFFSET) | (self.worker_id << WORKER_ID_OFFSET) | (self.datacenter_id << DATACENTER_ID_OFFSET) | sequence;

        Snowflake::from(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snowflake {
    value: i64,
}

impl Snowflake {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        let epoch_offset = self.value >> TIMESTAMP_OFFSET;
        chrono::Utc.timestamp_millis(epoch_offset + EPOCH)
    }
}

impl Into<i64> for Snowflake {
    fn into(self) -> i64 {
        self.value
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<DateTime<Utc>> for Snowflake {
    fn from(timestamp: DateTime<Utc>) -> Self {
        let epoch_offset = timestamp.timestamp_millis() - EPOCH;
        Self::new(epoch_offset << TIMESTAMP_OFFSET)
    }
}

impl Into<DateTime<Utc>> for Snowflake {
    fn into(self) -> DateTime<Utc> {
        self.timestamp()
    }
}