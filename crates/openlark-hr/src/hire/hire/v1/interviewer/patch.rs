//! 更新面试官信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interviewer/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::hire::hire::common_models::InterviewerOperationResult;

/// 更新面试官信息请求体（官方必填 `interviewer` 对象）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PatchBody {
    /// 面试官信息（必填）
    pub interviewer: InterviewerUpdateInfo,
}

/// 面试官更新信息（官方 `interviewer` 对象，当前仅支持更新认证状态）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct InterviewerUpdateInfo {
    /// 认证状态：1 未认证 / 2 已认证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_status: Option<i32>,
}

/// 更新面试官信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    interviewer_id: String,
    body: PatchBody,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            interviewer_id: String::new(),
            body: PatchBody::default(),
        }
    }

    /// 设置 `interviewer_id`。
    pub fn interviewer_id(mut self, interviewer_id: impl Into<String>) -> Self {
        self.interviewer_id = interviewer_id.into();
        self
    }

    /// 设置认证状态（官方 `interviewer.verify_status`：1 未认证 / 2 已认证）。
    pub fn verify_status(mut self, verify_status: i32) -> Self {
        self.body.interviewer.verify_status = Some(verify_status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        validate_required!(self.interviewer_id.trim(), "interviewer_id 不能为空");
        // 官方必填 interviewer 对象；当前仅支持更新认证状态，须指定 verify_status
        let _verify_status = self.body.interviewer.verify_status.ok_or_else(|| {
            openlark_core::error::validation_error(
                "interviewer.verify_status 不能为空".to_string(),
                "更新面试官认证状态时必须指定 verify_status".to_string(),
            )
        })?;

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/hire/v1/interviewers/{}",
            self.interviewer_id
        ))
        .body(serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新面试官信息响应数据为空",
        )
        .await
    }
}

/// 更新面试官信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatchResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段（官方响应 data 下唯一容器）。
    pub interviewer: Option<InterviewerOperationResult>,
}

impl ApiResponseTrait for PatchResponse {
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

    /// 端到端：PATCH /open-apis/hire/v1/interviewers/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/hire/v1/interviewers/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "interviewer": {
                        "user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
                        "verify_status": 1
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

        let resp = PatchRequest::new(config)
            .interviewer_id("test001".to_string())
            .verify_status(1)
            .execute()
            .await
            .expect("请求应成功");

        // 官方响应 data.interviewer 下含 user_id/verify_status（.md L70-72）
        let interviewer = resp.interviewer.as_ref().expect("interviewer 应反序列化");
        assert_eq!(
            interviewer.user_id.as_deref(),
            Some("ou_7dab8a3d3cdcc9da365777c7ad535d62")
        );
        assert_eq!(interviewer.verify_status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：interviewer 对象必填
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent, json!({ "interviewer": { "verify_status": 1 } }));
    }

    /// 缺 verify_status 时快速失败，不发起请求。
    #[tokio::test]
    async fn test_patch_rejects_missing_verify_status() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = PatchRequest::new(config)
            .interviewer_id("test001".to_string())
            .execute()
            .await
            .expect_err("缺 verify_status 应被拒绝");
        assert!(err.to_string().contains("verify_status"), "实际: {err}");
    }
}
