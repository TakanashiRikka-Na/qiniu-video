use redis::{Commands, RedisResult};

#[derive(Debug)]
pub(crate) struct RDB{
    rds:redis::Client,
}

impl RDB {
    pub  fn init_rdb()->Result<RDB,redis::RedisError>{
        let client = redis::Client::open("redis://127.0.0.1/")?;
        Ok( RDB{rds:client})
    }

    pub fn cache<T>(mut self, key:String, value:T) -> RedisResult<T> {
        let result = serde_json::to_string(&value)?;
        self.rds.set(key, result)?
    }
}
