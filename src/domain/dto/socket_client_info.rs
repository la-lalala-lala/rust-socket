use async_std::net::TcpStream;
use std::collections::HashMap;
use std::sync::RwLock;

// 定义客户端信息结构体
pub struct SocketClientInfo {
    pub stream: TcpStream,
    pub lock: RwLock<HashMap<u64, SocketClientInfo>>,
}