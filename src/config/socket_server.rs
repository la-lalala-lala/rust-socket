use std::collections::HashMap;
use std::net::Shutdown;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use std::sync::{Arc, RwLock};
use tokio::io::AsyncWriteExt;
use crate::config::SOCKET_CLIENTS;
use crate::domain::dto::socket_client_info::SocketClientInfo;

pub struct SocketServer {}


impl SocketServer {

    // 定义一个用于处理客户端连接的函数
    pub async fn init_socket_server(address:&str) -> std::io::Result<()>{
        // 接受客户端连接
        let listener = TcpListener::bind(address).await.unwrap();
        log::info!(" - Socket Server Local Address:   {}",address.replace("0.0.0.0", "127.0.0.1"));
        loop {
            // 接受客户端连接
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut reader = stream.clone();
            // 生成一个唯一的客户端 ID
            let id = rand::random::<u64>();
            // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
            let clients_rw = SOCKET_CLIENTS.clone();

            // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
            let mut clients = clients_rw.write().unwrap();

            // 将客户端信息加入到哈希表中
            clients.insert(id, SocketClientInfo {
                stream,
                lock: RwLock::new(HashMap::new()),
            });
            // 处理客户端连接
            async_std::task::spawn(async move {
                log::info!("Accepted from: {}", reader.peer_addr().unwrap());
                // 循环接收客户端消息
                loop {
                    // 读取客户端消息
                    let mut buf = [0u8; 1024];
                    let read_size = reader.read(&mut buf).await;
                    if read_size.is_err() {
                        log::error!("数据接收异常，可能是客户端主动断开连接");
                        break;
                    }
                    let size = read_size.unwrap();
                    if size == 0 {
                        log::error!("接收到客户端发来的空数据");
                        break;
                    }
                    let _message:String = String::from_utf8_lossy(&buf[..size]).to_string();
                    if _message.starts_with("address") {
                        // 接收到客户端是身份地址信息
                        println!("接收到客户端是身份地址信息");
                    }else{
                        // 打印客户端消息
                        println!("客户端 {} 发送消息: {}", id, _message);
                    }
                }
                log::info!("客户端断开连接");
                // 从哈希表中删除客户端连接信息
                let _clients_rw = SOCKET_CLIENTS.clone();
                // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
                let mut _clients = _clients_rw.write().unwrap();
                _clients.remove(&id);
                // 断开客户端连接
                let _ = reader.shutdown(Shutdown::Both).is_ok();
            });
        }
    }

    pub async fn close() {
        let id = 1;
        // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
        let clients = SOCKET_CLIENTS.clone();

        // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
        let mut clients = clients.write().unwrap();
        let mut reader = clients.get(&id).unwrap();
        let _ = reader.stream.shutdown(Shutdown::Both).is_ok();
    }

    pub async fn send() {
        let id = rand::random::<u64>();
        // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
        let clients = SOCKET_CLIENTS.clone();
        // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
        let mut clients = clients.write().unwrap();
        let mut reader = clients.get_mut(&id).unwrap();
        let message = String::from("你好");
        let _ = reader.stream.write(message.as_bytes()).await;
    }

}