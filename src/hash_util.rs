use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn rh<T: Hash>(value: T) {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish() as i32;
    tracing::info!("【hash值是: {} 】", hash);
}

pub fn jh(value: &str) {
    let bytes = value.as_bytes();
    let mut h: i64 = 0;
    for byte in bytes {
        h = 31 * h + i64::from(*byte);
    }
    tracing::info!("【（该算法仅仅模仿，不是直接调用jni程序）hash值是: {} 】", h);
}

