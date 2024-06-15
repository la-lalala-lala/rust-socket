use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::io;

use crate::util;
use serde::de::Visitor;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

/// A generic error that represents all the ways a method can fail inside of rexpr::core.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Default Error
    E(String, i32),
}

impl Display for Error {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(error, code) => write!(f, "{}:{}", code, error),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Error::from(err.to_string())
    }
}

/// 用户没有指定状态码时，默认util::FAIL
impl From<i32> for Error {
    fn from(arg: i32) -> Self {
        match arg {
            util::constant::NOT_AUTHORIZE_CODE => Error::E(
                String::from("not authorize error"),
                util::constant::NOT_AUTHORIZE_CODE,
            ),
            util::constant::TOKEN_ERROR_CODE => Error::E(String::from("token error"), util::constant::TOKEN_ERROR_CODE),
            _ => Error::E(String::from("other error"), arg),
        }
    }
}

/// 用户没有指定状态码时，默认util::FAIL
impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return Error::E(arg.to_string(), util::constant::FAIL_CODE);
    }
}

/// 用户没有指定状态码时，默认util::FAIL
impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return Error::E(arg, util::constant::FAIL_CODE);
    }
}

impl From<(&str, i32)> for Error {
    fn from(arg: (&str, i32)) -> Self {
        return Error::E(arg.0.parse().unwrap(), arg.1);
    }
}

impl From<(std::string::String, i32)> for Error {
    fn from(arg: (std::string::String, i32)) -> Self {
        return Error::E(arg.0, arg.1);
    }
}

impl From<Error> for std::io::Error {
    fn from(arg: Error) -> Self {
        arg.into()
    }
}

/// 为防止敏感信息泄露，std框架产生的异常需要对外脱敏，在这里赋予特殊的状态码
impl From<&dyn std::error::Error> for Error {
    fn from(arg: &dyn std::error::Error) -> Self {
        return Error::E(arg.to_string(), util::constant::UNKNOWN_ERROR_CODE);
    }
}

/// 为防止敏感信息泄露，rbatis框架产生的异常需要对外脱敏，在这里赋予特殊的状态码
impl From<rbatis::error::Error> for Error {
    fn from(arg: rbatis::error::Error) -> Self {
        Error::E(arg.to_string(), util::constant::UNKNOWN_ERROR_CODE)
    }
}

/// 为防止敏感信息泄露，actix_web框架产生的异常需要对外脱敏，在这里赋予特殊的状态码
impl From<actix_web::error::Error> for Error {
    fn from(arg: actix_web::error::Error) -> Self {
        Error::E(arg.to_string(), util::constant::UNKNOWN_ERROR_CODE)
    }
}

/// 重写clone方法，否则将造成code丢失
impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::E(message, code) => Error::E(message.to_string(), *code),
            // _ => {
            //     Error::from(self.to_string())
            // }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match source {
            Error::E(message, code) => {
                *self = Error::E(message.to_string(), *code);
            }
            //_ => {
            //    *self = Error::from(self.to_string());
            //}
        }
    }
}

// This is what #[derive(Serialize)] would generate.
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
        where
            E: std::error::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: std::error::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let r = deserializer.deserialize_string(ErrorVisitor)?;
        return Ok(Error::from(r));
    }
}