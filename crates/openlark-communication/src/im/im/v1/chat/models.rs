//! 群相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 群列表排序方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSortType {
    /// 按创建时间升序。
    ByCreateTimeAsc,
    /// 按活跃时间降序。
    ByActiveTimeDesc,
}

impl ChatSortType {
    /// 返回请求参数使用的字符串值。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ByCreateTimeAsc => "ByCreateTimeAsc",
            Self::ByActiveTimeDesc => "ByActiveTimeDesc",
        }
    }
}

/// 群分享链接有效期
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatLinkValidityPeriod {
    /// 一周有效。
    Week,
    /// 一年有效。
    Year,
    /// 永久有效。
    Permanently,
}

impl ChatLinkValidityPeriod {
    /// 返回请求参数使用的字符串值。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Week => "week",
            Self::Year => "year",
            Self::Permanently => "permanently",
        }
    }
}

/// 获取群分享链接响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatLinkResponse {
    /// 群分享链接。
    pub share_link: String,
    /// 过期时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 是否为永久链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permanent: Option<bool>,
}

impl ApiResponseTrait for GetChatLinkResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
