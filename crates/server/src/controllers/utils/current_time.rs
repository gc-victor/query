use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_millis() -> i64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let since_epoch = since_epoch.as_millis();
    let since_epoch_str = since_epoch.to_string();
    let since_epoch = &since_epoch_str[0..10];

    since_epoch.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_time_millis() {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let current_time = current_time_millis();

        assert_eq!(current_time.to_string(), since_epoch.to_string()[0..10]);
    }
}
