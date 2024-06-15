use std::collections::HashMap;
use std::net::Shutdown;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use tokio::io::AsyncWriteExt;


// 定义客户端信息结构体
struct ClientInfo {
    stream: TcpStream,
    lock: RwLock<HashMap<u64, ClientInfo>>,
}

// 定义一个 Arc<RwLock<HashMap<u64, ClientInfo>>> 类型的全局变量
lazy_static! {
    static ref CLIENTS: Arc<RwLock<HashMap<u64, ClientInfo>>> = Arc::new(RwLock::new(HashMap::new()));
}
// 定义一个用于处理客户端连接的函数
async fn socket_handle() -> std::io::Result<()>{
    // 接受客户端连接
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    loop {
        // 接受客户端连接
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut reader = stream.clone();
        // 生成一个唯一的客户端 ID
        let id = 1;//rand::random::<u64>();
        // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
        let clients_rw = CLIENTS.clone();

        // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
        let mut clients = clients_rw.write().unwrap();

        // 将客户端信息加入到哈希表中
        clients.insert(id, ClientInfo {
            stream,
            lock: RwLock::new(HashMap::new()),
        });
        // 处理客户端连接
        async_std::task::spawn(async move {
            println!("Accepted from: {}", reader.peer_addr().unwrap());
            // 循环接收客户端消息
            loop {
                // 读取客户端消息
                let mut buf = [0u8; 1024];
                let read_size = reader.read(&mut buf).await;
                if read_size.is_err() {
                    println!("数据接收异常，可能是客户端主动断开连接");
                    break;
                }
                let size = read_size.unwrap();
                if size == 0 {
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
            println!("client close");
            // 从哈希表中删除客户端连接信息
            let _clients_rw = CLIENTS.clone();
            // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
            let mut _clients = _clients_rw.write().unwrap();
            _clients.remove(&id);
            // 断开客户端连接
            let _ = reader.shutdown(Shutdown::Both).is_ok();
        });

    }
}

pub async fn close(req: HttpRequest) -> HttpResponse {
    let id = 1;//rand::random::<u64>();
    // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
    let clients = CLIENTS.clone();

    // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
    let mut clients = clients.write().unwrap();
    let mut reader = clients.get(&id).unwrap();
    let _ = reader.stream.shutdown(Shutdown::Both).is_ok();
    // 返回响应
    HttpResponse::Ok().body("OK")
}

pub async fn send(req: HttpRequest) -> HttpResponse {
    let id = 1;//rand::random::<u64>();
    // 使用 Arc<RwLock<HashMap<u64, ClientInfo>>> 来获取 CLIENTS
    let clients = CLIENTS.clone();

    // 使用 RwLock::write().await 来获取 CLIENTS 的写锁
    let mut clients = clients.write().unwrap();
    let mut reader = clients.get_mut(&id).unwrap();
    let message = String::from("你好");
    let _ = reader.stream.write(message.as_bytes()).await;
    // 返回响应
    HttpResponse::Ok().body("OK")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //task::block_on(socket_handle());
    // 启动 Socket 服务器
    let socket_server = socket_handle();
    let actix_server = HttpServer::new(move || {
        App::new()
            .route("/close", web::get().to(close))
            .route("/send", web::get().to(send))
    })
        .bind("127.0.0.1:8001")?
        .run();

    tokio::try_join!(socket_server, actix_server)?;
    Ok(())
}