use chrono::{DateTime, Utc};
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建 Exchange 账户绑定请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExchangeBindingRequestBody {
    /// 用户 ID
    pub user_id: String,
    /// Exchange 账户邮箱
    pub exchange_account: String,
    /// Exchange 服务器地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_server_url: Option<String>,
    /// Exchange 账户密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_password: Option<String>,
    /// 同步设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_setting: Option<ExchangeSyncSetting>,
}

/// Exchange 同步设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeSyncSetting {
    /// 是否同步日历事件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_calendar_event: Option<bool>,
    /// 同步方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_direction: Option<String>,
    /// 同步时间范围（天）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_time_range_days: Option<i32>,
}

/// Exchange 账户绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExchangeBindingResponse {
    /// 绑定 ID
    pub exchange_binding_id: String,
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Exchange 账户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_account: Option<String>,
    /// 绑定状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
}

/// 查询 Exchange 账户绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExchangeBindingResponse {
    /// 绑定 ID
    pub exchange_binding_id: String,
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Exchange 账户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_account: Option<String>,
    /// 绑定状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime<Utc>>,
    /// 同步状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<i32>,
}

/// 删除 Exchange 账户绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteExchangeBindingResponse {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl ApiResponseTrait for CreateExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
