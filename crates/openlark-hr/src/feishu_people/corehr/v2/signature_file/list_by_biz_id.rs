//! 根据流程获取电子签文件信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/list_by_biz_id>

use super::models::SignatureFile;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 根据流程获取电子签文件信息请求
#[derive(Debug, Clone)]
pub struct ListByBizIdRequest {
    /// 配置信息
    config: Config,
    /// 业务自定义流程 ID（必填，唯一不重复，不超过 100 字符）
    biz_process_id: String,
    /// 业务类型（必填）：开放平台发起的流程传 OpenAPI，BPM 发起的流程传 Bpm
    biz_type: String,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
    /// 是否需要返回签署链接
    select_sign_url: Option<bool>,
}

impl ListByBizIdRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            biz_process_id: String::new(),
            biz_type: String::new(),
            user_id_type: None,
            select_sign_url: None,
        }
    }

    /// 设置业务自定义流程 ID（必填）。
    pub fn biz_process_id(mut self, biz_process_id: impl Into<String>) -> Self {
        self.biz_process_id = biz_process_id.into();
        self
    }

    /// 设置业务类型（必填，OpenAPI / Bpm）。
    pub fn biz_type(mut self, biz_type: impl Into<String>) -> Self {
        self.biz_type = biz_type.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置是否需要返回签署链接。
    pub fn select_sign_url(mut self, select_sign_url: bool) -> Self {
        self.select_sign_url = Some(select_sign_url);
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
        validate_required!(self.biz_process_id, "biz_process_id 不能为空");
        validate_required!(self.biz_type, "biz_type 不能为空");

        let request = ApiRequest::<ListByBizIdResponse>::get(
            "/open-apis/corehr/v2/signature_files/list_by_biz_id",
        )
        .query("biz_process_id", self.biz_process_id)
        .query("biz_type", self.biz_type)
        .query_opt("user_id_type", self.user_id_type)
        .query_opt(
            "select_sign_url",
            self.select_sign_url.map(|v| v.to_string()),
        );

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
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/signature_files/list_by_biz_id
    /// （官方查询参数 biz_process_id/biz_type 必填；data 内层键为 signature_files / batch_sign_url）
    #[tokio::test]
    async fn test_list_by_biz_id_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/signature_files/list_by_biz_id"))
            .and(query_param("biz_process_id", "129b83f5-c023-4e85-b2c7-80ce777e57fd"))
            .and(query_param("biz_type", "OpenAPI"))
            .and(query_param("select_sign_url", "true"))
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
            .biz_process_id("129b83f5-c023-4e85-b2c7-80ce777e57fd")
            .biz_type("OpenAPI")
            .select_sign_url(true)
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
