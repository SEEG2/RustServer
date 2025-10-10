use std::time::{SystemTime, UNIX_EPOCH};

pub fn random_non_zero_u32() -> u32 { 
    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let nanos = now.as_nanos();
        let value = (nanos ^ (nanos >> 32)) as u32;
        if value != 0 {
            return value;
        }
    }
}