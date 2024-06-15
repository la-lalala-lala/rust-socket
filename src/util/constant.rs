/// 定义的业务相关处理回执码
/// 处理成功
pub const SUCCESS_CODE: i32 = 0;
/// 处理失败（通用）
pub const FAIL_CODE: i32 = -1;
/// 记录不存在
pub const NOT_EXIST_CODE: i32 = -2;
/// 没有授权
pub const NOT_AUTHORIZE_CODE: i32 = -3;
/// 缺少参数
pub const NOT_PARAMETER_CODE: i32 = -4;
/// token错误
pub const TOKEN_ERROR_CODE: i32 = -5;
/// 文件错误
pub const FILE_IO_ERROR_CODE: i32 = -6;
/// 错误的请求
pub const BAD_REQUEST_ERROR_CODE: i32 = -7;
/// 未知的错误类型（由内部意外抛出的，框架）
pub const UNKNOWN_ERROR_CODE: i32 = -404;

/// 定义数据目录下的子级目录
/// 对外公众访问的根目录
pub const PUBLIC_VIEW_ROOT_PATH: &str = "warehouse";
/// 数据库目录
pub const DATABASE_PATH: &str = "database";
/// 文档目录
pub const DOCUMENT_PATH: &str = "document/file";
/// logo目录
pub const LOGO_PATH: &str = "picture/logo";
/// 插图目录
pub const ILLUSTRATED_PATH: &str = "picture/illustrated";
/// 墙纸&背景目录
pub const WALLPAPER_PATH: &str = "picture/wallpaper";

/// 定义日期相关的格式化format
pub const FORMAT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";
pub const FORMAT_Y_M_D_T_H_M_S: &str = "%Y-%m-%dT%H:%M:%S";
pub const FORMAT_Y_M_D_T_H_M_S_Z: &str = "%Y-%m-%dT%H:%M:%S%z";
pub const FORMAT_Y_M_D: &str = "%Y-%m-%d";
pub const FORMAT_YMD: &str = "%Y%m%d";

/// 定义已登录用户的缓存前缀
pub const USER_CACHE_PREFIX: &str = "login";
/// 定义防重复请求token的缓存前缀
pub const REQUEST_TOKEN_PREFIX: &str = "request";
/// 定义浏览器端token的过期时间，单位：秒
pub const BROWSER_PLATFORM_TTL: u64 = 3600;
/// 定义桌面端token的过期时间，单位：秒
pub const DESKTOP_PLATFORM_TTL: u64 = 604800;
/// 定义防重复请求token的缓存前缀
pub const WECHAT_ACCESS_TOKEN_PREFIX: &str = "wechat_access_token";