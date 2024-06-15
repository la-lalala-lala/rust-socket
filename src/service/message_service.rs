use std::collections::HashMap;
use actix_web::HttpRequest;
use log::error;
use crate::dao::user_mapper::UserMapper;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::dto::user::{UserDTO, UserPageDTO};
use crate::domain::entity::User;
use crate::domain::vo::user::UserVO;
use crate::{primary_rbatis_pool, util};
use crate::util::result::Result;
use crate::util::error::Error;
use crate::util::page::Page;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::password_encoder_util::PasswordEncoder;
use serde_json::{Map, Value};
use crate::util::result;

pub struct MessageService {}

impl MessageService {

    /// 发送微信消息
    /// param account 用户openid
    /// param template 消息模板id
    pub async fn send_wechat_message(&self,openid: &str,template: &str,arg: &Value)-> Result<bool>{
        Err(Error::from(("用户openid不能为空!", util::NOT_PARAMETER_CODE, )))
    }



    /// 发送邮件消息
    /// param mail     用户邮件地址
    /// param template 消息模板id
    pub async fn send_mail_message(&self,mail: &str,template: &str,arg: &Value)-> Result<bool>{
        Err(Error::from(("用户openid不能为空!", util::NOT_PARAMETER_CODE, )))
    }

    // /// 用户分页
    // pub async fn user_page(&self, arg: &UserPageDTO) -> Result<Page<UserVO>> {
    //     let mut extend = ExtendPageDTO {
    //         page_no: arg.page_no,
    //         page_size: arg.page_size,
    //         begin_time: arg.begin_time.clone(),
    //         end_time: arg.end_time.clone(),
    //     };
    //     let count_result = UserMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
    //     if count_result.is_err() {
    //         error!("在用户分页统计时，发生异常:{}", count_result.unwrap_err());
    //         return Err(Error::from("用户分页查询异常"));
    //     }
    //     let total_row = count_result.unwrap().unwrap();
    //     if total_row <= 0 {
    //         return Err(Error::from((
    //             "未查询到符合条件的数据",
    //             NOT_EXIST_CODE,
    //         )));
    //     }
    //     let mut result = Page::<UserVO>::page_query(total_row, &extend);
    //     // 重新设置limit起始位置
    //     extend.page_no = Some((result.page_no - 1) * result.page_size);
    //     extend.page_size = Some(result.page_size);
    //     let page_result = UserMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
    //     if page_result.is_err() {
    //         error!(
    //             "在用户分页获取页面数据时，发生异常:{}",
    //             page_result.unwrap_err()
    //         );
    //         return Err(Error::from("用户分页查询异常"));
    //     }
    //     let page_rows = page_result.unwrap();
    //     let mut list = vec![];
    //     for item in page_rows.unwrap() {
    //         list.push(UserVO::from(item));
    //     }
    //     result.records = Some(list);
    //     return Ok(result);
    // }
    //
    // ///创建账号
    // pub async fn user_add(&self, arg: &UserDTO) -> Result<u64> {
    //     let check_flag = arg.account.is_none()
    //         || arg.account.as_ref().unwrap().is_empty()
    //         || arg.name.is_none()
    //         || arg.name.as_ref().unwrap().is_empty()
    //         || arg.email.is_none()
    //         || arg.email.as_ref().unwrap().is_empty()
    //         || arg.phone.is_none()
    //         || arg.phone.as_ref().unwrap().is_empty()
    //         || arg.organize_id.is_none();
    //     if check_flag {
    //         return Err(Error::from((
    //             "账号、姓名、手机号、邮箱以及所属组织不能为空!",
    //             NOT_PARAMETER_CODE,
    //         )));
    //     }
    //
    //     let query_user_wrap =
    //         User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
    //     if query_user_wrap.is_err() {
    //         error!("查询用户异常：{}", query_user_wrap.unwrap_err());
    //         return Err(Error::from("查询用户失败!"));
    //     }
    //     let old_user = query_user_wrap.unwrap().into_iter().next();
    //     if old_user.is_some() {
    //         return Err(Error::from(format!(
    //             "账户:{}已存在!",
    //             arg.account.as_ref().unwrap()
    //         )));
    //     }
    //     let mut password = arg.password.clone().unwrap_or_default();
    //     if password.is_empty() {
    //         //默认密码
    //         password = "123456".to_string();
    //     }
    //     let user = User {
    //         account: arg.account.clone(),
    //         name: arg.name.clone(),
    //         password: PasswordEncoder::encode(&password).into(),
    //         sex: arg.sex.clone(),
    //         qq: arg.qq.clone(),
    //         email: arg.email.clone(),
    //         phone: arg.phone.clone(),
    //         birthday: arg.birthday.clone(),
    //         hometown: arg.hometown.clone(),
    //         autograph: arg.autograph.clone(),
    //         logo: arg.logo.clone(),
    //         background: arg.background,
    //         organize_id: arg.organize_id,
    //         state: 1.into(),
    //         create_time: DateTimeUtil::naive_date_time_to_str(
    //             &Some(DateUtils::now()),
    //             &FORMAT_Y_M_D_H_M_S,
    //         ),
    //         update_time: None,
    //     };
    //     let write_result = User::insert(primary_rbatis_pool!(), &user).await;
    //     if write_result.is_err() {
    //         error!("创建账号时，发生异常:{}", write_result.unwrap_err());
    //         return Err(Error::from("创建账号时，发生异常!"));
    //     }
    //     return Ok(write_result?.rows_affected);
    // }
    //
    // /// 修改用户信息
    // pub async fn user_edit(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
    //     if arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() {
    //         return Err(Error::from((
    //             "账号account不能为空!",
    //             NOT_PARAMETER_CODE,
    //         )));
    //     }
    //     // 首先判断要修改的用户是否存在
    //     let query_user_wrap =
    //         User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
    //     if query_user_wrap.is_err() {
    //         error!("查询用户异常：{}", query_user_wrap.unwrap_err());
    //         return Err(Error::from("查询用户失败!"));
    //     }
    //     let user_warp = query_user_wrap.unwrap().into_iter().next();
    //     let user_exist = user_warp.ok_or_else(|| {
    //         Error::from((
    //             format!("账号:{} 不存在!", &arg.account.clone().unwrap()),
    //             NOT_EXIST_CODE,
    //         ))
    //     })?;
    //
    //     let user_edit = User {
    //         account: user_exist.account,
    //         name: arg.name.clone(),
    //         password: if arg.password.is_some() {
    //             Some(PasswordEncoder::encode(arg.password.as_ref().unwrap()))
    //         } else {
    //             user_exist.password
    //         },
    //         sex: arg.sex.clone(),
    //         qq: arg.qq.clone(),
    //         email: arg.email.clone(),
    //         phone: arg.phone.clone(),
    //         birthday: arg.birthday.clone(),
    //         hometown: arg.hometown.clone(),
    //         autograph: arg.autograph.clone(),
    //         logo: arg.logo.clone(),
    //         background: arg.background,
    //         organize_id: arg.organize_id,
    //         state: arg.state,
    //         create_time: user_exist.create_time,
    //         update_time: DateTimeUtil::naive_date_time_to_str(
    //             &Some(DateUtils::now()),
    //             &FORMAT_Y_M_D_H_M_S,
    //         ),
    //     };
    //     let result = UserMapper::update_user(primary_rbatis_pool!(), &user_edit).await; //CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
    //     if result.is_err() {
    //         error!(
    //             "在修改用户{}的信息时，发生异常:{}",
    //             arg.account.as_ref().unwrap(),
    //             result.unwrap_err()
    //         );
    //         return Err(Error::from(format!(
    //             "修改账户[{}]信息失败!",
    //             arg.account.as_ref().unwrap()
    //         )));
    //     }
    //     Ok(result.unwrap().rows_affected)
    // }
    //
    // /// 删除用户
    // pub async fn user_remove(&self, account: &str) -> Result<u64> {
    //     if account.is_empty() {
    //         return Err(Error::from((
    //             "account 不能为空！",
    //             NOT_PARAMETER_CODE,
    //         )));
    //     }
    //     let r = User::delete_by_account(primary_rbatis_pool!(), account.clone()).await?;
    //     return Ok(r.rows_affected);
    // }
    //
    // /// 用户详情
    // pub async fn user_detail(&self, arg: &UserDTO) -> Result<UserVO> {
    //     let account = arg.account.clone().unwrap_or_default();
    //     let query_user_wrap =
    //         User::select_by_account(primary_rbatis_pool!(), &account.clone()).await;
    //     if query_user_wrap.is_err() {
    //         error!("查询用户异常：{}", query_user_wrap.unwrap_err());
    //         return Err(Error::from("查询用户失败!"));
    //     }
    //     let user_warp = query_user_wrap.unwrap().into_iter().next();
    //     let user = user_warp.ok_or_else(|| {
    //         Error::from((
    //             format!("账号:{} 不存在!", &account.clone()),
    //             NOT_EXIST_CODE,
    //         ))
    //     })?;
    //     let user_vo = UserVO::from(user);
    //     return Ok(user_vo);
    // }


}