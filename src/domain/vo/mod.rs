pub mod user;

/// 响应模块

use actix_http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::util::error::Error;
use crate::util::constant::{NOT_EXIST_CODE,TOKEN_ERROR_CODE,NOT_AUTHORIZE_CODE};

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(crate::util::constant::SUCCESS_CODE),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            let err: Error = arg.clone().err().unwrap();
            match err {
                Error::E(message, code) => Self {
                    code: Some(code),
                    msg: Some(message.to_string()),
                    data: None,
                },
            }
        }
    }

    pub fn resp_json(&self) -> HttpResponse {
        let code: i32 = self.code.clone().unwrap();
        match code {
            NOT_EXIST_CODE => {
                return HttpResponse::build(StatusCode::NOT_FOUND)
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .insert_header(("Cache-Control", "no-cache"))
                    .insert_header(("Content-Type", "text/json;charset=UTF-8"))
                    .body(self.to_string());
            }
            TOKEN_ERROR_CODE => {
                return HttpResponse::build(StatusCode::LOCKED)
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .insert_header(("Cache-Control", "no-cache"))
                    .insert_header(("Content-Type", "text/json;charset=UTF-8"))
                    .body(self.to_string());
            }
            NOT_AUTHORIZE_CODE => {
                return HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .insert_header(("Cache-Control", "no-cache"))
                    .insert_header(("Content-Type", "text/json;charset=UTF-8"))
                    .body(self.to_string());
            }
            _ => {
                return HttpResponse::Ok()
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .insert_header(("Cache-Control", "no-cache"))
                    .insert_header(("Content-Type", "text/json;charset=UTF-8"))
                    .body(self.to_string());
            }
        }
    }
}

impl<T> ToString for RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}