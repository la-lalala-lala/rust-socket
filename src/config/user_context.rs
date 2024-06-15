use crate::config::CONTEXT;
use crate::util::error::Error;
use actix_http::header::HeaderValue;
use actix_web::HttpRequest;
use log::error;
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::util::constant::{USER_CACHE_PREFIX,NOT_AUTHORIZE_CODE,TOKEN_ERROR_CODE,FAIL_CODE};

/// 用户上下文
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UserContext {
    // 账号
    pub account: String,
    // 姓名
    pub name: String,
    // 组织
    pub organize: u64,
    // 登录ip
    pub ip: String,
    // 登录城市
    pub city: String,
    // 会话有效期
    pub leeway: u64,
}

impl UserContext {
    /// extract token detail
    /// secret: your secret string
    pub async fn extract_token(token: &str) -> Result<UserContext, Error> {
        let user_cache = CONTEXT
            .redis_client
            .get_string(&format!("{:}:{:}", &USER_CACHE_PREFIX, token))
            .await?;
        let user_data: UserContext = serde_json::from_str(user_cache.as_str()).unwrap();
        return Ok(user_data);
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_token_by_header(
        token: Option<&HeaderValue>,
    ) -> Result<UserContext, Error> {
        return match token {
            Some(token) => {
                let token: &str = token.to_str().unwrap_or("");
                UserContext::extract_token(token).await
            }
            _ => {
                error!("access_token is empty!");
                return Err(Error::from(NOT_AUTHORIZE_CODE));
            }
        };
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_user_by_header(token: Option<&HeaderValue>) -> Option<UserContext> {
        let extract_result = &UserContext::extract_token_by_header(token).await;
        if extract_result.is_err() {
            error!(
                "在获取用户信息时，发生异常:{}",
                extract_result.clone().unwrap_err().to_string()
            );
            return None;
        }
        let user_session = extract_result.clone().unwrap();
        return Some(user_session);
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_user_by_request(req: &HttpRequest) -> Option<UserContext> {
        let token = req.headers().get("access_token");
        UserContext::extract_user_by_header(token).await
    }

    /// create token
    /// secret: your secret string
    pub async fn create_token(account: &str) -> Result<String, Error> {
        // 生成用户token，在这里用account:雪花算法拼接，好处是，在判断用户是否已经登录时，直接通过account:判断
        let token = format!("{:}:{:}", account, &Snowflake::default().generate());
        return Ok(token);
    }

    /// verify token invalid
    /// secret: your secret string
    pub async fn verify(token: &str) -> Result<UserContext, Error> {
        let key = format!("{:}:{:}", &USER_CACHE_PREFIX, token);
        return match CONTEXT.redis_client.get_string(&key).await {
            Ok(val) => {
                match val.is_empty() {
                    false => {
                        // 反序列化成对象
                        let user_data: UserContext = serde_json::from_str(val.as_str()).unwrap();
                        // 完成一次续期
                        CONTEXT
                            .redis_client
                            .set_ex(&key, Some(Duration::from_secs(user_data.leeway)))
                            .await;
                        Ok(user_data)
                    }
                    true => {
                        error!("InvalidToken! token={}", token);
                        return Err(Error::from(TOKEN_ERROR_CODE));
                    }
                }
            }
            Err(err) => {
                error!("check redis user token cache data fail! token:{:?}", err);
                return Err(Error::from(("InvalidToken other errors", FAIL_CODE)));
            }
        };
    }
}