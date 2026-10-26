//! 综合更新候选人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/combined_update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CombinedUpdateRequestBody, CombinedUpdateResponse};

/// 综合更新候选人请求
#[derive(Debug, Clone)]
pub struct CombinedUpdateRequest {
    /// 配置信息
    config: Config,
    request_body: CombinedUpdateRequestBody,
}

impl CombinedUpdateRequest {
    /// 创建请求（`talent_id` 与 `basic_info` 必填）
    pub fn new(config: Config, request_body: CombinedUpdateRequestBody) -> Self {
        Self {
            config,
            request_body,
        }
    }

    /// 设置请求体。
    pub fn request_body(mut self, request_body: CombinedUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedUpdateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 验证必填字段
        validate_required!(self.request_body.talent_id.trim(), "人才 ID 不能为空");

        let api_endpoint = HireApiV1::TalentCombinedUpdate;
        let request = ApiRequest::<CombinedUpdateResponse>::post(api_endpoint.to_url()).body(
            serde_json::to_value(&self.request_body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?,
        );

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "综合更新候选人响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for CombinedUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hire::hire::v1::talent::models::TalentCombinedBasicInfo;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/talents/combined_update，请求体按官方形状（talent_id + basic_info 必填）
    #[tokio::test]
    async fn test_combined_update_sends_official_body_shape() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": { "result": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CombinedUpdateRequest::new(
            config,
            CombinedUpdateRequestBody::new(
                "7158793341499426348".to_string(),
                TalentCombinedBasicInfo {
                    name: Some("张三".to_string()),
                    email: Some("16xx1@126.com".to_string()),
                    ..Default::default()
                },
            ),
        )
        .execute()
        .await
        .expect("请求应成功");
        assert!(resp.result);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：talent_id + basic_info 嵌套结构
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({
                "talent_id": "7158793341499426348",
                "basic_info": { "name": "张三", "email": "16xx1@126.com" }
            })
        );
    }

    /// 缺 talent_id 时快速失败，不发起请求。
    #[tokio::test]
    async fn test_combined_update_rejects_blank_talent_id() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CombinedUpdateRequest::new(
            config,
            CombinedUpdateRequestBody::new("  ", TalentCombinedBasicInfo::default()),
        )
        .execute()
        .await
        .expect_err("缺 talent_id 应被拒绝");
        assert!(err.to_string().contains("人才 ID"), "实际: {err}");
    }
}
