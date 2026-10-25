//! 查询猎头保护期信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/agency/protect_search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// 猎头保护期（官方 `agency_protection`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyProtection {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 保护期类型：1 人才保护期 / 2 职位保护期。
    pub protection_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 职位保护期所在的投递 ID。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 保护期开始时间，毫秒时间戳。
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 保护期过期时间，毫秒时间戳。
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 猎头供应商 ID。
    pub agency_supplier_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 猎头供应商名称。
    pub agency_supplier_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 猎头顾问 ID。
    pub agency_supplier_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 猎头顾问名称。
    pub agency_supplier_user_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 查询猎头保护期信息请求
#[derive(Debug, Clone)]
pub struct ProtectSearchRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ProtectSearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ProtectSearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProtectSearchResponse> {
        let mut request = ApiRequest::<ProtectSearchResponse>::post(
            "/open-apis/hire/v1/agencies/protection_period/search",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询猎头保护期信息响应数据为空",
        )
        .await
    }
}

/// 查询猎头保护期信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProtectSearchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否已入职。
    pub is_onboarded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否在猎头保护期内入职。
    pub onboarded_in_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 入职时所在保护期，当且仅当 is_onboarded=true 且 onboarded_in_protection=true 时有值。
    pub onboarded_protection: Option<AgencyProtection>,
    #[serde(default)]
    /// 保护期列表，若返回空，则表明人才上没有任何的保护期。
    pub protection_list: Vec<AgencyProtection>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ProtectSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/agencies/protection_period/search
    #[tokio::test]
    async fn test_protect_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/agencies/protection_period/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "is_onboarded": false,
                    "onboarded_in_protection": false,
                    "onboarded_protection": {
                        "protection_type": 1,
                        "application_id": "6930815272790114323",
                        "start_time": "1700023694629",
                        "expire_time": "1700023694630",
                        "agency_supplier_id": "6930815272790114324",
                        "agency_supplier_name": { "zh_cn": "超越猎头公司", "en_us": "chaoyue head-hunting company" },
                        "agency_supplier_user_id": "6930815272790114324",
                        "agency_supplier_user_name": { "zh_cn": "张三", "en_us": "zhangsan" }
                    },
                    "protection_list": [
                        {
                            "protection_type": 2,
                            "application_id": "6930815272790114323",
                            "start_time": "1700023694629",
                            "expire_time": "1700023694630",
                            "agency_supplier_id": "6930815272790114324",
                            "agency_supplier_name": { "zh_cn": "超越猎头公司", "en_us": "chaoyue head-hunting company" },
                            "agency_supplier_user_id": "6930815272790114324",
                            "agency_supplier_user_name": { "zh_cn": "张三", "en_us": "zhangsan" }
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ProtectSearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        // 官方 data 形状：is_onboarded/onboarded_in_protection/onboarded_protection/protection_list
        assert_eq!(resp.is_onboarded, Some(false));
        assert_eq!(resp.onboarded_in_protection, Some(false));
        let onboarded = resp
            .onboarded_protection
            .as_ref()
            .expect("onboarded_protection 应反序列化");
        assert_eq!(onboarded.protection_type, Some(1));
        assert_eq!(
            onboarded
                .agency_supplier_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("超越猎头公司")
        );
        assert_eq!(resp.protection_list.len(), 1);
        assert_eq!(resp.protection_list[0].protection_type, Some(2));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
