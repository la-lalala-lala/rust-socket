use rbatis::executor::Executor;
use rbatis::{crud, html_sql, impl_delete, impl_select, impl_select_page, impled};
use rbatis::rbdc::db::ExecResult;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::dto::user::UserPageDTO;
use crate::domain::entity::User;

crud!(User {});
impl_select!(User{select_by_account(account:&str) => "`where account = #{account}`"});
impl_delete!(User{delete_by_account(account:&str) => "`where account = #{account}`"});
impl_select_page!(User{select_page(name:&str,account:&str)=>
    "`where del = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    if account != '':
      ` and account like #{'%'+account+'%'}`
    if !sql.contains('count'):
     ` order by create_date desc`"});

pub struct UserMapper {}

impl UserMapper {
    /// 修改用户信息
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn update_user(rb: &mut dyn Executor, user: &User) -> rbatis::Result<ExecResult> {
        impled!()
    }

    /// 分页查询用户
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_page(
        rb: &mut dyn Executor,
        user: &UserPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<Vec<User>>, rbatis::Error> {
        impled!()
    }

    /// 查询用户总数
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_count(
        rb: &mut dyn Executor,
        user: &UserPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<u64>, rbatis::Error> {
        impled!()
    }

}