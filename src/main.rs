use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use uuid::Uuid;

// Lazy static global variable for WebSocket sessions
lazy_static! {
    static ref WEBSOCKET_SESSIONS: Arc<Mutex<HashMap<String, Addr<MyWebSocket>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Define WebSocket actor
struct MyWebSocket {
    client_id: String,
    sessions: Arc<Mutex<HashMap<String, Addr<MyWebSocket>>>>,
}

// MyWebSocket 实现了 Actor trait，定义了 WebSocket 的行为和生命周期管理。
impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Store the WebSocket address in sessions map
        println!("有客户端[{}]加入",self.client_id.clone());
        self.sessions.lock().unwrap().insert(self.client_id.clone(), ctx.address());
    }

    // 连接断开处理
    fn stopped(&mut self, ctx: &mut Self::Context) {
        // 当客户端主动或意外断开 WebSocket 连接时，Actix 框架会自动调用 MyWebSocket 的 stopped 方法。
        // 在 stopped 方法中，会从 sessions 中移除断开连接的客户端信息，确保不会继续保留已断开连接的状态。
        // Remove client identifier from sessions map
        println!("有客户端[{}]断开",self.client_id.clone());
        self.sessions.lock().unwrap().remove(&self.client_id);
    }
}

// MyWebSocket 实现了 StreamHandler<Result<ws::Message, ws::ProtocolError>> trait，处理从客户端接收到的消息。
// 当收到文本消息时，会在控制台打印消息内容，并使用 ctx.text(text) 将消息发送回客户端（即实现了回显功能）。
// 当收到二进制消息时，会在控制台打印相关信息，但不做处理。
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message from client {}: {}", self.client_id, text);
                // Example: Echo back the received message
                ctx.text(text);
            }
            Ok(ws::Message::Binary(_)) => {
                println!("Received binary message from client {}", self.client_id);
                // Example: Ignore binary messages
            }
            _ => (),
        }
    }
}

// WebSocket message
#[derive(Debug, Message)]
#[rtype(result = "()")]
struct Message {
    client_id: String,
    msg: String,
}

// 在 Actix 中，实现 Handler<Message> trait 是用来处理发送给 Actor 的消息，而不是发送消息。
// 一、实现 Handler<Message> trait:
// 1、当一个 Actor 实现了 Handler<Message> trait 时，它定义了如何处理接收到的特定类型的消息。
// 2、这个 handle 方法会在 Actor 接收到该类型的消息时被 Actix 框架调用。
// 二、消息处理:
// 1、在 handle 方法中，可以编写逻辑来处理接收到的消息，例如对消息进行验证、记录日志、更改状态，或者做出其他响应。
// 2、如果需要回复消息给发送者或广播消息给其他连接的客户端，可以在 handle 方法中调用 Actor 的方法或者使用 ctx 对象发送消息。
// 三、发送消息:
// 1、如果要向其他 Actor 发送消息，应该使用该 Actor 的 Addr<MyActor> 对象的 send() 方法。
// 2、这个方法会将消息发送到目标 Actor 的消息队列中，等待目标 Actor 处理。

impl Handler<Message> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        // Handle the incoming message
        println!("Sending message to client {}: {}", msg.client_id, msg.msg);
        // 调用了 ctx.text(msg.msg) 来将接收到的消息内容发送回客户端
        ctx.text(msg.msg);
    }
}

// HTTP handler to initiate WebSocket connection
// 在 ws_handler 函数中，生成一个唯一的 client_id（使用 Uuid::new_v4().to_string()），并创建一个新的 MyWebSocket Actor 实例，将其作为 WebSocket 的处理程序启动。
async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    // 生成一个唯一的客户端表示
    let client_id = Uuid::new_v4().to_string();
    // Extract sessions from web::Data
    let sessions_inner = WEBSOCKET_SESSIONS.clone();
    // 创建一个新的 MyWebSocket Actor 实例
    let actor = MyWebSocket {
        client_id: client_id.clone(),
        sessions: sessions_inner.clone(),
    };
    // Start the WebSocket handshake
    let resp = ws::start(actor, &req, stream)?;
    Ok(resp)
}


// HTTP handler to send message to specific client
async fn send_message(client_id: web::Path<String>, msg: web::Bytes) -> HttpResponse {
    // 根据 client_id 从 WEBSOCKET_SESSIONS 中获取对应的 Addr<MyWebSocket>，然后使用 addr.send(Message { ... }).await 异步发送消息给 WebSocket Actor 处理。
    let client_id = client_id.into_inner();
    if let Some(addr) = WEBSOCKET_SESSIONS.lock().unwrap().get(&client_id) {
        // Send message to client by sending a message to the WebSocket actor
        let text = String::from_utf8_lossy(&msg).to_string();
        addr.send(Message {
            client_id: client_id.clone(),
            msg: text,
        })
            .await
            .unwrap(); // Send message asynchronously
        HttpResponse::Ok().body("Message sent to client")
    } else {
        HttpResponse::NotFound().body("Client not found")
    }
}


// HTTP handler to broadcast message to all clients
async fn broadcast_message(msg: web::Bytes) -> HttpResponse {
    let message = String::from_utf8_lossy(&msg).to_string();
    // Iterate over all WebSocket clients and broadcast message
    WEBSOCKET_SESSIONS.lock().unwrap().values().for_each(|addr| {
        addr.do_send(Message {
            client_id: "server".to_owned(),  // Using "server" as pseudo client_id for broadcast
            msg: message.clone(),
        });
    });
    HttpResponse::Ok().body("Message broadcasted to all clients")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建了一个 sessions 变量，类型为 Arc<Mutex<HashMap<String, Addr<MyWebSocket>>>>，用于存储所有 WebSocket 客户端的地址。
    let sessions: Arc<Mutex<HashMap<String, Addr<MyWebSocket>>>> = Arc::new(Mutex::new(HashMap::new()));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            //.app_data(web::Data::new(sessions.clone()))
            // WebSocket endpoint
            .route("/ws", web::get().to(ws_handler))
            // HTTP endpoint to send message to client
            .route("/send/{client_id}", web::post().to(send_message))
            .route("/broadcast", web::post().to(broadcast_message))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
