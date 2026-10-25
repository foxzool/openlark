//! 操作员工离职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/submit>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 操作员工离职请求
#[derive(Debug, Clone)]
pub struct SubmitRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl SubmitRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubmitResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubmitResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::OffboardingSubmit;
        let request = ApiRequest::<SubmitResponse>::post(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "操作员工离职响应数据为空",
        )
        .await
    }
}

/// 操作员工离职响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化），
/// 官方 `data` 为扁平对象，字段直接平铺在本结构体上。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitResponse {
    /// 离职记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 雇员 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 离职原因唯一标识，可通过【查询员工离职原因列表】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    /// 离职日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 离职原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_explanation: Option<String>,
    /// 是否加入离职屏蔽名单
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_block_list: Option<bool>,
    /// 屏蔽原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    /// 屏蔽原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

impl ApiResponseTrait for SubmitResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/offboardings/submit
    #[tokio::test]
    async fn test_submit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/offboardings/submit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "offboarding_id": "7095671727698478604",
                    "employment_id": "6982509313466189342",
                    "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
                    "offboarding_date": "2022-05-18",
                    "offboarding_reason_explanation": "离职原因说明",
                    "add_block_list": false,
                    "block_reason": "红线",
                    "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
                    "created_time": "2022-05-09 17:50:17"
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

        let resp = SubmitRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.offboarding_id.as_deref(), Some("7095671727698478604"));
        assert_eq!(
            resp.offboarding_reason_unique_identifier.as_deref(),
            Some("reason_for_offboarding_option8")
        );
        assert_eq!(resp.add_block_list, Some(false));
        assert_eq!(resp.block_reason.as_deref(), Some("红线"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
