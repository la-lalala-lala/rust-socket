use crate::config::user_context::UserContext;
use crate::config::CONTEXT;

/// 权限校验模块


///Whether the interface is in the whitelist
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if path.starts_with(x) {
            return true;
        }
    }
    return false;
}

///Check whether the token is valid and has not expired
pub async fn checked_token(
    token: &str,
    _path: &str,
) -> Result<UserContext, crate::util::error::Error> {
    //check token alive
    let check = UserContext::verify(token).await;
    match check {
        Ok(context) => {
            return Ok(context);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

///Permission to check
pub async fn check_auth(token: &UserContext, path: &str) -> Result<(), crate::util::error::Error> {
    return Ok(());
    // 先不做权限菜单的校验
    // let sys_res = CONTEXT.sys_res_service.finds_all().await?;
    // for token_permission in &token.permissions {
    //     for x in &sys_res {
    //         match &x.inner.permission {
    //             Some(permission) => match &x.inner.path {
    //                 None => {}
    //                 Some(x_path) => {
    //                     if permission.eq(token_permission) && path.contains(x_path) {
    //                         return Ok(());
    //                     }
    //                 }
    //             },
    //             _ => {}
    //         }
    //     }
    // }
    // return Err(crate::util::error::Error::from("无权限访问!"));
}