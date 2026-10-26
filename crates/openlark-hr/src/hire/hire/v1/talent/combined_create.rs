//! 综合创建候选人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/combined_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CombinedCreateRequestBody, CombinedCreateResponse};

/// 综合创建候选人请求
#[derive(Debug, Clone)]
pub struct CombinedCreateRequest {
    /// 配置信息
    config: Config,
    request_body: CombinedCreateRequestBody,
}

impl CombinedCreateRequest {
    /// 创建请求（`basic_info` 必填，其内 `name` 必填）
    pub fn new(config: Config, request_body: CombinedCreateRequestBody) -> Self {
        Self {
            config,
            request_body,
        }
    }

    /// 设置请求体。
    pub fn request_body(mut self, request_body: CombinedCreateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedCreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedCreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 验证必填字段：basic_info.name
        let name = self
            .request_body
            .basic_info
            .name
            .clone()
            .unwrap_or_default();
        validate_required!(name.trim(), "basic_info.name 不能为空");

        let api_endpoint = HireApiV1::TalentCombinedCreate;
        let request = ApiRequest::<CombinedCreateResponse>::post(api_endpoint.to_url()).body(
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
            "综合创建候选人响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for CombinedCreateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/talents/combined_create，请求体按官方形状（basic_info 必填）
    #[tokio::test]
    async fn test_combined_create_sends_official_body_shape() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_create"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": { "talent_id": "7017428969192859009" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let basic_info = TalentCombinedBasicInfo {
            name: Some("小明".to_string()),
            mobile: Some("182900291190".to_string()),
            mobile_country_code: Some("CN_1".to_string()),
            email: Some("16xx1@126.com".to_string()),
            ..Default::default()
        };
        let resp = CombinedCreateRequest::new(
            config,
            CombinedCreateRequestBody::new(basic_info)
                .resume_source_id("1".to_string())
                .folder_id_list(vec!["6942778198054125570".to_string()]),
        )
        .execute()
        .await
        .expect("请求应成功");
        assert_eq!(resp.talent_id, "7017428969192859009");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：basic_info 嵌套结构
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({
                "resume_source_id": "1",
                "folder_id_list": ["6942778198054125570"],
                "basic_info": {
                    "name": "小明",
                    "mobile": "182900291190",
                    "mobile_country_code": "CN_1",
                    "email": "16xx1@126.com"
                }
            })
        );
    }

    /// basic_info.name 缺失时快速失败，不发起请求。
    #[tokio::test]
    async fn test_combined_create_rejects_missing_name() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CombinedCreateRequest::new(
            config,
            CombinedCreateRequestBody::new(TalentCombinedBasicInfo::default()),
        )
        .execute()
        .await
        .expect_err("缺 basic_info.name 应被拒绝");
        assert!(err.to_string().contains("name"), "实际: {err}");
    }
}
