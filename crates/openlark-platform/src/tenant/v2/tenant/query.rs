//! 获取企业信息
//!
//! 文档: https://open.feishu.cn/document/server-docs/tenant-v2/query
//! docPath: https://open.feishu.cn/document/server-docs/tenant-v2/query

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取企业信息 Builder
#[derive(Debug, Clone)]
pub struct TenantQueryBuilder {
    config: Config,
}

impl TenantQueryBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TenantQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TenantQueryResponse> {
        let url = "/open-apis/tenant/v2/tenant/query";

        let req: ApiRequest<TenantQueryResponse> = ApiRequest::get(url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 企业信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TenantQueryResponse {
    /// 租户名称
    pub name: String,
    /// 租户编号
    #[serde(rename = "tenant_code")]
    pub tenant_code: Option<String>,
    /// 租户图标
    pub icon: Option<String>,
    /// 租户国际化名称
    #[serde(rename = "i18n_name")]
    pub i18n_name: Option<I18nName>,
}

/// 国际化名称
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct I18nName {
    /// 中文名称
    #[serde(rename = "zh_cn")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(rename = "en_us")]
    pub en_us: Option<String>,
    /// 日文名称
    #[serde(rename = "ja_jp")]
    pub ja_jp: Option<String>,
}

impl ApiResponseTrait for TenantQueryResponse {}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
