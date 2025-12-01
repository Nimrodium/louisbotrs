use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, TimeDelta, TimeZone, Utc};

pub type LouisEpoch = u64;
pub type UnixEpoch = DateTime<Utc>;
pub static LOUIS_EPOCH: LazyLock<UnixEpoch> =
    LazyLock::new(|| Utc.with_ymd_and_hms(2025, 5, 14, 0, 0, 0).unwrap());

pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

pub fn now_louis_epoch() -> LouisEpoch {
    unix_to_epoch(Utc::now())
}

pub fn epoch_to_unix(e: LouisEpoch) -> UnixEpoch {
    *LOUIS_EPOCH + TimeDelta::days(e as i64)
}

pub fn unix_to_epoch(u: UnixEpoch) -> LouisEpoch {
    let diff = u - *LOUIS_EPOCH;
    if diff.num_days() > -1 {
        diff.num_days() as LouisEpoch
    } else {
        0
    }
}
