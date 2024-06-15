use rust_socket::controller::message_controller;
use rust_socket::config::CONTEXT;
use rust_socket::middleware::actix_interceptor::ActixInterceptor;
use rust_socket::config::scheduler::Scheduler;

use actix_web::{web, App,HttpServer};
use rust_socket::config::socket_server::SocketServer;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 日志初始化
    rust_socket::config::logger::init_log();
    // 数据库连接池初始化
    CONTEXT.init_pool().await;
    // 调度组件初始化
    Scheduler::init_system_scheduler().await;
    let actix_server = HttpServer::new(|| {
        App::new()
            .wrap(ActixInterceptor {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            //.route("/backend/login", web::post().to(system_controller::login))
            //.route("/backend/logout", web::post().to(system_controller::logout))
            // 映射静态资源目录
            //.service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/message")
                    .service(message_controller::send_wechat_message)
                    .service(message_controller::send_mail_message)
                    // .service(message_controller::user_add)
                    // .service(message_controller::user_update)
                    // .service(message_controller::user_detail)
                    // .service(message_controller::user_remove)
                    // .service(message_controller::user_page)
            )
    }).bind(&CONTEXT.config.server_url)?.run();
    let socket_server = SocketServer::init_socket_server(&CONTEXT.config.socket_url);
    tokio::try_join!(socket_server, actix_server)?;
    Ok(())
}