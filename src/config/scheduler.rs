use crate::config::{CONTEXT, SCHEDULER};
use delay_timer::prelude::{Task, TaskBuilder, TaskError};
use log::{error, info};
use crate::util::constant::{WECHAT_ACCESS_TOKEN_PREFIX};

/// 调度任务 https://github.com/BinChengZhao/delay-timer
pub struct Scheduler {}

/// 落实mysql 的 mysqldump操作
// pub async fn execute_mysqldump_body() {
//     WeChatApi::take_access_token().await;
// }
//
// /// 构造MySQL备份计划任务
// fn build_mysqldump_async_task() -> Result<Task, TaskError> {
//     let mut task_builder = TaskBuilder::default();
//     task_builder.set_frequency_repeated_by_cron_str("0 15 */2 * * ?").set_task_id(0).set_maximum_running_time(300).spawn_async_routine(execute_mysqldump_body)
// }

impl Scheduler {
    /// 初始化系统级别的调度任务（发生在系统每次启动时）
    pub async fn init_system_scheduler() {
        //let scheduler = SCHEDULER.lock().await;
        // 添加一个备份数据库的定时任务
        //scheduler.add_task(build_mysqldump_async_task().unwrap());
        info!(" - cron pool init finish!");
    }
}