//! 妙记 API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};

/// Minutes API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum MinutesApiV1 {
    /// 获取妙记信息
    Get(String),
    /// 订阅妙记变更事件
    Subscription,
    /// 取消订阅妙记变更事件
    Unsubscription,
    /// 下载妙记音视频文件
    MediaGet(String),
    /// 导出妙记文字记录
    TranscriptGet(String),
    /// 获取妙记统计数据
    StatisticsGet(String),
}

impl MinutesApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            MinutesApiV1::Get(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}")
            }
            MinutesApiV1::Subscription => "/open-apis/minutes/v1/minutes/subscription".to_string(),
            MinutesApiV1::Unsubscription => {
                "/open-apis/minutes/v1/minutes/unsubscription".to_string()
            }
            MinutesApiV1::MediaGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/media")
            }
            MinutesApiV1::TranscriptGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/transcript")
            }
            MinutesApiV1::StatisticsGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/statistics")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for MinutesApiV1 {
    fn to_url(&self) -> String {
        MinutesApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::Get(_) | Self::MediaGet(_) | Self::TranscriptGet(_) | Self::StatisticsGet(_) => {
                HttpMethod::Get
            }
            Self::Subscription | Self::Unsubscription => HttpMethod::Post,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// 不扩展公开 `MinutesApiV1` 的补充端点，避免破坏下游穷举匹配。
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[cfg(feature = "minutes")]
pub(crate) enum MinutesExtraApiV1 {
    /// 搜索妙记。
    Search,
    /// 获取妙记 AI 产物。
    Artifacts(String),
}

#[cfg(feature = "minutes")]
impl MinutesExtraApiV1 {
    /// 返回配置了稳定请求语义的请求。
    pub(crate) fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

#[cfg(feature = "minutes")]
impl CatalogEndpoint for MinutesExtraApiV1 {
    fn to_url(&self) -> String {
        match self {
            Self::Search => "/open-apis/minutes/v1/minutes/search".to_string(),
            Self::Artifacts(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{minute_token}/artifacts")
            }
        }
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::Search => HttpMethod::Post,
            Self::Artifacts(_) => HttpMethod::Get,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::test_support::catalog_semantics_snapshot;

    #[test]
    fn minutes_catalog_semantics_snapshots() {
        insta::assert_snapshot!(
            "minutes_catalog_semantics",
            catalog_semantics_snapshot::<MinutesApiV1>()
        );
        #[cfg(feature = "minutes")]
        insta::assert_snapshot!(
            "minutes_extra_catalog_semantics",
            catalog_semantics_snapshot::<MinutesExtraApiV1>()
        );
    }
}
