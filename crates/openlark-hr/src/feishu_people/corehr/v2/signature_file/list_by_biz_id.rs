//! 根据流程获取电子签文件信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/list_by_biz_id>

use super::models::SignatureFile;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据流程获取电子签文件信息请求
#[derive(Debug, Clone)]
pub struct ListByBizIdRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl ListByBizIdRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListByBizIdResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListByBizIdResponse> {
        let mut request = ApiRequest::<ListByBizIdResponse>::get(
            "/open-apis/corehr/v2/signature_files/list_by_biz_id",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 根据流程获取电子签文件信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListByBizIdResponse {
    /// 返回的流程关联的电子签文件列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_files: Option<Vec<SignatureFile>>,
    /// 批量签署链接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_sign_url: Option<String>,
}

impl ApiResponseTrait for ListByBizIdResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/signature_files/list_by_biz_id
    /// （官方 data 内层键为 signature_files / batch_sign_url）
    #[tokio::test]
    async fn test_list_by_biz_id_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/signature_files/list_by_biz_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "signature_files": [
                        {
                            "signature_file_id": "7147527056140813828",
                            "names": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "create_time": "2023-01-01 00:00:00",
                            "update_time": "2023-01-01 00:00:00",
                            "employment_id": "7123132668099919891",
                            "pre_hire_id": "7123132668099919891",
                            "signature_file_state": {
                                "enum_name": "pending_sign",
                                "display": [ { "lang": "zh-CN", "value": "待签字" } ]
                            },
                            "contract_code": "48793beea04f4e2583d50732e481d507",
                            "effective_date": "2024-12-01",
                            "template_id": "7147527056140813828",
                            "sign_url": "https://www.letsign.com/v2/saas/sign?signCode=05325cb58b77484f92d4b145c18222cf"
                        }
                    ],
                    "batch_sign_url": "https://www.letsign.com/v2/saas/sign?signCode=05325cb58b77484f92d4b145c18222cf"
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

        let resp = ListByBizIdRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let files = resp.signature_files.expect("signature_files 应解码");
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].signature_file_id.as_deref(),
            Some("7147527056140813828")
        );
        assert_eq!(files[0].pre_hire_id.as_deref(), Some("7123132668099919891"));
        assert_eq!(
            files[0]
                .signature_file_state
                .as_ref()
                .and_then(|s| s.enum_name.clone()),
            Some("pending_sign".to_string())
        );
        assert!(resp.batch_sign_url.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
