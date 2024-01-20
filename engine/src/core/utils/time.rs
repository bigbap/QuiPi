use std::time::{
    SystemTime,
    SystemTimeError
};

pub fn now_milis() -> Result<u128, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis())
}
pub fn now_secs() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}
