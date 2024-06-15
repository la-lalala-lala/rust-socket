use super::error::Error;


// 定义返回的结构
pub type Result<T> = std::result::Result<T, Error>;