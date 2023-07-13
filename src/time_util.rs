use chrono::{FixedOffset, TimeZone, Utc};

pub fn conversion(str: &str) {
    let regex: Vec<&str> = str.split("-").collect();
    if regex.len() < 2 {
        long_to_string(str);
        return;
    }
    string_to_long(str);
}

fn string_to_long(str: &str) {
    let datetime = FixedOffset::east_opt(8 * 3600).unwrap()
        .datetime_from_str(str, "%Y-%m-%d %H:%M:%S")
        .unwrap();
    tracing::info!("时间戳: {} 】", datetime.timestamp_millis());
}

fn long_to_string(str: &str) {
    let beijing_time_zone = FixedOffset::east_opt(8 * 3600).unwrap();
    if str.eq("default") || str.is_empty() {
        let now = Utc::now();
        let beijing_time = now.with_timezone(&beijing_time_zone);
        tracing::info!("【时间戳: {}, 时间: {} 】", beijing_time, beijing_time.timestamp_millis());
        return;
    }
    let num = match str.parse::<i64>() {
        Ok(num) => num,
        Err(_) => {
            tracing::error!("str parse to i64 fail");
            return;
        }
    };
    let utc_date_time = Utc.timestamp_millis_opt(num).unwrap();
    let beijing_date_time = utc_date_time.with_timezone(&beijing_time_zone);
    tracing::info!("【时间: {} 】", beijing_date_time);
}