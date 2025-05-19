use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn compute_fingerprint(msg: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    msg.hash(&mut hasher);
    hasher.finish()
}
