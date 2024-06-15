/// 数据库操作模块

pub mod user_mapper;

use rbatis::RBatis;
use crate::config::ApplicationConfig;

pub fn init_rbatis(config: &ApplicationConfig) -> RBatis {
    let rbatis = RBatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}