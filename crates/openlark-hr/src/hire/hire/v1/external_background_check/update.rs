//! 更新外部背调
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_background_check/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalBackgroundCheck;

/// 更新外部背调请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    external_background_check_id: String,
    request_body: Option<Value>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            external_background_check_id: String::new(),
            request_body: None,
        }
    }

    /// 提供 `external_background_check_id` 能力。
    pub fn external_background_check_id(
        mut self,
        external_background_check_id: impl Into<String>,
    ) -> Self {
        self.external_background_check_id = external_background_check_id.into();
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        validate_required!(
            self.external_background_check_id.trim(),
            "external_background_check_id 不能为空"
        );

        let mut request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/external_background_checks/{}",
            self.external_background_check_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新外部背调响应数据为空",
        )
        .await
    }
}

/// 更新外部背调响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `external_background_check` 字段（官方响应 data 下唯一容器）。
    pub external_background_check: Option<ExternalBackgroundCheck>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/hire/v1/external_background_checks/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/hire/v1/external_background_checks/test001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "external_background_check": {
                        "id": "6989202908470446380",
                        "external_application_id": "7003247299220982060",
                        "date": 1626602069393i64,
                        "name": "张三的背调",
                        "result": "已通过",
                        "attachment_list": [
                            { "id": "6987954043925432620", "name": "test_resume.pdf", "size": 2827182 }
                        ]
                    }
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

        let resp = UpdateRequest::new(config)
            .external_background_check_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        // 官方响应 data.external_background_check 容器（.md L63-72）
        let check = resp
            .external_background_check
            .as_ref()
            .expect("external_background_check 应反序列化");
        assert_eq!(check.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(check.result.as_deref(), Some("已通过"));
        assert_eq!(
            check
                .attachment_list
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.size),
            Some(2827182)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
