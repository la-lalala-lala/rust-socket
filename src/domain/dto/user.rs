use serde::{Deserialize, Serialize};

/// 用户通用数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDTO {
    /// 用户
    pub account: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// qq号
    pub qq: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 电话号码
    pub phone: Option<String>,
    /// 生日
    pub birthday: Option<String>,
    /// 故乡
    pub hometown: Option<String>,
    /// 签名
    pub autograph: Option<String>,
    /// 头像地址
    pub logo: Option<String>,
    /// 设置的背景
    pub background: Option<u64>,
    /// 所属组织
    pub organize_id: Option<u64>,
    /// 是否锁定(1正常，2锁定)
    pub state: Option<u32>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
    /// 会话token
    pub token: Option<String>,
}

impl UserDTO {
    // 初始化一个空的数据
    pub fn empty() -> Self {
        Self {
            account: None,
            name: None,
            password: None,
            sex: None,
            qq: None,
            email: None,
            phone: None,
            birthday: None,
            hometown: None,
            autograph: None,
            logo: None,
            background: None,
            organize_id: None,
            state: None,
            create_time: None,
            update_time: None,
            token: None,
        }
    }
}

/// 用户分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organize_id: Option<u64>,
    pub state: Option<u32>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}