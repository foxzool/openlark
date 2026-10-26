//! 修改勋章信息 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update>

use super::create::BadgeI18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改勋章信息的请求构建器。
pub struct UpdateBadgeRequestBuilder {
    badge_id: String,
    name: String,
    explanation: Option<String>,
    detail_image: String,
    show_image: String,
    i18n_name: Option<BadgeI18nText>,
    i18n_explanation: Option<BadgeI18nText>,
    config: Config,
}

impl UpdateBadgeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            name: String::new(),
            explanation: None,
            detail_image: String::new(),
            show_image: String::new(),
            i18n_name: None,
            i18n_explanation: None,
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置名称（租户内唯一，最多 30 个字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置描述文案（最多 100 个字符）。
    pub fn explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = Some(explanation.into());
        self
    }

    /// 设置详情图 Key（需使用 1h 内上传的图片 key）。
    pub fn detail_image(mut self, detail_image: impl Into<String>) -> Self {
        self.detail_image = detail_image.into();
        self
    }

    /// 设置头像挂饰图 Key（需使用 1h 内上传的图片 key）。
    pub fn show_image(mut self, show_image: impl Into<String>) -> Self {
        self.show_image = show_image.into();
        self
    }

    /// 设置多语言名称。
    pub fn i18n_name(mut self, i18n_name: BadgeI18nText) -> Self {
        self.i18n_name = Some(i18n_name);
        self
    }

    /// 设置多语言描述文案。
    pub fn i18n_explanation(mut self, i18n_explanation: BadgeI18nText) -> Self {
        self.i18n_explanation = Some(i18n_explanation);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeResponse> {
        validate_required!(self.badge_id, "勋章 ID 不能为空");
        validate_required!(self.name, "勋章名称不能为空");
        validate_required!(self.detail_image, "勋章详情图 Key 不能为空");
        validate_required!(self.show_image, "勋章头像挂饰图 Key 不能为空");

        let url = format!("/open-apis/admin/v1/badges/{}", self.badge_id);
        let request_body = UpdateBadgeBody {
            name: self.name,
            explanation: self.explanation,
            detail_image: self.detail_image,
            show_image: self.show_image,
            i18n_name: self.i18n_name,
            i18n_explanation: self.i18n_explanation,
        };
        let api_request: ApiRequest<UpdateBadgeResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "修改勋章信息").await
    }
}

/// 修改勋章请求体
#[derive(Debug, Serialize)]
pub struct UpdateBadgeBody {
    /// 租户内唯一的勋章名称，最多 30 个字符
    pub name: String,
    /// 勋章的描述文案，最多 100 个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// 企业勋章的详情图 Key
    pub detail_image: String,
    /// 企业勋章的头像挂饰图 Key
    pub show_image: String,
    /// 勋章的多语言名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<BadgeI18nText>,
    /// 勋章的多语言描述文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_explanation: Option<BadgeI18nText>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章信息的响应。
pub struct UpdateBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UpdateBadgeRequestBuilder, will be removed in v1.0 (#271)")]
pub type UpdateBadgeBuilder = UpdateBadgeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PUT .../admin/v1/badges/{badge_id} → 强类型 UpdateBadgeResponse。
    #[tokio::test]
    async fn test_update_badge_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/admin/v1/badges/badge_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "badge_id": "badge_001",
                    "name": "优秀员工",
                    "description": "更新后的描述",
                    "update_time": "1717000000"
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

        let resp = UpdateBadgeRequestBuilder::new(config)
            .badge_id("badge_001")
            .name("优秀员工")
            .explanation("更新后的描述")
            .detail_image("75a1949f-d9df-4b46-bc88-dacc51e88f3j")
            .show_image("03daa74a-159f-49e9-963e-b6c4d76103fj")
            .execute()
            .await
            .expect("修改勋章信息应成功");
        assert_eq!(resp.badge_id, "badge_001");
        assert_eq!(resp.name, "优秀员工");
        assert_eq!(resp.update_time, "1717000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "PUT");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/admin/v1/badges/badge_001"
        );
        // 请求体须与官方文档一致：name/detail_image/show_image 必填
        let body: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(body["name"], "优秀员工");
        assert_eq!(body["detail_image"], "75a1949f-d9df-4b46-bc88-dacc51e88f3j");
        assert_eq!(body["show_image"], "03daa74a-159f-49e9-963e-b6c4d76103fj");
        assert_eq!(body["explanation"], "更新后的描述");
    }
}
