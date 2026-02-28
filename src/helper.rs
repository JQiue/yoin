use std::{collections::HashMap, hash::Hash};

use helpers::hash::md5;
use migration::prelude::{DateTime, Utc};
use tokio::sync::Mutex;

pub fn generate_avatar(email: &str) -> String {
  format!("https://cravatar.cn/avatar/{}", md5(email))
}

pub struct RateLimiter<T> {
  cache: Mutex<HashMap<T, DateTime<Utc>>>,
}

impl<T: Eq + PartialEq + Hash> RateLimiter<T> {
  pub fn new() -> Self {
    Self {
      cache: Mutex::new(HashMap::new()),
    }
  }

  pub async fn check_rate_limit(&self, key: T, limit_seconds: i64) -> bool {
    let mut cache = self.cache.lock().await;
    let now = Utc::now();
    if let Some(last_comment_time) = cache.get(&key) {
      let elapsed = now.signed_duration_since(*last_comment_time).num_seconds();
      if elapsed < limit_seconds {
        return true;
      }
    }
    cache.insert(key, now);
    false
  }
}
