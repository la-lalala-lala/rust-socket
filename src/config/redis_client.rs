use std::time::Duration;

use crate::util::error::Error;
use crate::util::result::Result;
use log::error;
use redis::aio::Connection;
use redis::{AsyncCommands, RedisResult};

/// Redis客户端操作工具
pub struct RedisClient {
    pub client: redis::Client,
}

impl RedisClient {
    pub fn new(url: &str) -> Self {
        println!("[rust_socket] conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        println!("[rust_socket] conncect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisClient connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::util::error::Error::from(err));
        }
        return Ok(conn.unwrap());
    }

    pub async fn exists(&self, k: &str) -> Result<bool> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<bool>> = conn.exists(&k).await;
        return match result {
            Ok(v) => Ok(v.unwrap()),
            Err(e) => Err(Error::from(format!(
                "RedisClient exists({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        return self.set_string_ex(&k, &v, None).await;
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
        return match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(Error::from(format!(
                "RedisClient get_string({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    ///set_string Automatically expire
    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        let k = k.to_string();
        let v = v.to_string();
        let mut conn = self.get_conn().await?;
        return if ex.is_none() {
            match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisClient set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        } else {
            match redis::cmd("SET")
                .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                .query_async(&mut conn)
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisClient set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        };
    }

    ///set_string Automatically expire
    pub async fn ttl(&self, k: &str) -> Result<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisClient ttl fail:{}",
                e.to_string()
            ))),
        };
    }

    /// 设置过期时间，单位：秒
    pub async fn set_ex(&self, k: &str, ex: Option<Duration>) -> Result<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        return match redis::cmd("EXPIRE")
            .arg(&[&k, &ex.unwrap().as_secs().to_string()])
            .query_async(&mut conn)
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisClient expire fail:{}",
                e.to_string()
            ))),
        };
    }

    /// 使用scan模糊获取指定前缀的key，数量限制在20个以内
    pub async fn scan(&self, k: &str) -> Result<Vec<String>> {
        let k = format!("{}*", k);
        let mut result = Vec::new();
        let mut conn = self.get_conn().await?;
        // scan 0 match login:shmily* count 10
        let cmd_result: RedisResult<(i64, Vec<String>)> = redis::cmd("SCAN")
            .arg(&["0", "MATCH", &k, "COUNT", "20"])
            .query_async::<Connection, (i64, Vec<String>)>(&mut conn)
            .await;
        return match cmd_result {
            Ok((next_cursor, keys)) => {
                if 0 == next_cursor && !keys.is_empty() && keys.len() > 0 {
                    result.extend(keys);
                }
                Ok(result)
            }
            Err(e) => Err(Error::from(format!(
                "RedisClient scan fail:{}",
                e.to_string()
            ))),
        };
    }

    /// 删除指定的key
    pub async fn delete(&self, k: &str) -> Result<i64> {
        let k = k.to_string();
        let mut conn = self.get_conn().await?;
        return match redis::cmd("DEL").arg(&k).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisClient del fail:{}",
                e.to_string()
            ))),
        };
    }

    /// 批量删除指定的key
    pub async fn batch_delete(&self, keys: &Vec<String>) -> Result<i64> {
        if keys.is_empty() {
            return Ok(-1);
        }
        let mut conn = self.get_conn().await?;
        return match redis::cmd("DEL").arg(&keys).query_async(&mut conn).await {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisClient del fail:{}",
                e.to_string()
            ))),
        };
    }
}