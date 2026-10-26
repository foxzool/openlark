//! 终止电子签文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/terminate>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 终止电子签文件请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateBody {
    /// 欲终止的电子签文件 id 列表（必填，最多 100 个）
    pub ids: Vec<String>,
    /// 操作人 ID（必填），ID 格式取决于查询参数 user_id_type
    pub operator: String,
    /// 终止原因（必填，不超过 300 字符）
    pub terminate_reason: String,
}

/// 终止电子签文件请求
#[derive(Debug, Clone)]
pub struct TerminateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: TerminateBody,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl TerminateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: TerminateBody::default(),
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: TerminateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置欲终止的电子签文件 id 列表（必填）。
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.body.ids = ids;
        self
    }

    /// 设置操作人 ID（必填）。
    pub fn operator(mut self, operator: impl Into<String>) -> Self {
        self.body.operator = operator.into();
        self
    }

    /// 设置终止原因（必填，不超过 300 字符）。
    pub fn terminate_reason(mut self, terminate_reason: impl Into<String>) -> Self {
        self.body.terminate_reason = terminate_reason.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TerminateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TerminateResponse> {
        validate_required!(self.body.ids, "ids 不能为空");
        validate_required!(self.body.operator, "operator 不能为空");
        validate_required!(self.body.terminate_reason, "terminate_reason 不能为空");

        let request =
            ApiRequest::<TerminateResponse>::post("/open-apis/corehr/v2/signature_files/terminate")
                .query_opt("user_id_type", self.user_id_type)
                .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 终止电子签文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminateResponse {
    /// 欲终止的文件总数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 成功总数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    /// 失败总数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_count: Option<i64>,
    /// 终止成功的文件 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_file_id_list: Option<Vec<String>>,
    /// 终止失败的文件 id 和对应的原因列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_file_id_and_reasons: Option<Vec<TerminateSignatureFailIdAndReason>>,
}

/// 终止失败的文件 id 和原因（官方类型 terminate_signature_fail_id_and_reason）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminateSignatureFailIdAndReason {
    /// 终止操作失败的文件 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    /// 终止失败的原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
}

impl ApiResponseTrait for TerminateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/signature_files/terminate
    /// （官方请求体必填 ids/operator/terminate_reason）
    #[tokio::test]
    async fn test_terminate_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/signature_files/terminate"))
            .and(query_param("user_id_type", "people_corehr_id"))
            .and(body_json(json!({
                "ids": ["7410063506638112300"],
                "operator": "7386593105085988xxx",
                "terminate_reason": "请求流程发生变化，需要重新签署，因此终止签署流程中的文件"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "total_count": 2,
                    "success_count": 1,
                    "fail_count": 1,
                    "success_file_id_list": ["7410063506638112300"],
                    "fail_file_id_and_reasons": [
                        {
                            "signature_file_id": "7345403447803379731",
                            "fail_reason": "文件状态不允许终止"
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

        let resp = TerminateRequest::new(config)
            .ids(vec!["7410063506638112300".to_string()])
            .operator("7386593105085988xxx")
            .terminate_reason("请求流程发生变化，需要重新签署，因此终止签署流程中的文件")
            .user_id_type("people_corehr_id")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.total_count, Some(2));
        assert_eq!(resp.success_count, Some(1));
        assert_eq!(resp.fail_count, Some(1));
        assert_eq!(
            resp.success_file_id_list,
            Some(vec!["7410063506638112300".to_string()])
        );
        let failures = resp.fail_file_id_and_reasons.expect("应解析失败列表");
        assert_eq!(failures.len(), 1);
        assert_eq!(
            failures[0].signature_file_id,
            Some("7345403447803379731".to_string())
        );
        assert_eq!(
            failures[0].fail_reason,
            Some("文件状态不允许终止".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
