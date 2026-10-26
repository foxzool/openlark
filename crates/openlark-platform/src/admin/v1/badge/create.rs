//! 创建勋章 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create>
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create-2>

use crate::common::api_endpoints::AdminApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建勋章请求
pub struct CreateBadgeRequestBuilder {
    name: String,
    explanation: Option<String>,
    detail_image: String,
    show_image: String,
    i18n_name: Option<BadgeI18nText>,
    i18n_explanation: Option<BadgeI18nText>,
    config: Config,
}

impl CreateBadgeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            name: String::new(),
            explanation: None,
            detail_image: String::new(),
            show_image: String::new(),
            i18n_name: None,
            i18n_explanation: None,
            config,
        }
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
    pub async fn execute(self) -> SDKResult<CreateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateBadgeResponse> {
        validate_required!(self.name, "勋章名称不能为空");
        validate_required!(self.detail_image, "勋章详情图 Key 不能为空");
        validate_required!(self.show_image, "勋章头像挂饰图 Key 不能为空");

        let request_body = CreateBadgeBody {
            name: self.name,
            explanation: self.explanation,
            detail_image: self.detail_image,
            show_image: self.show_image,
            i18n_name: self.i18n_name,
            i18n_explanation: self.i18n_explanation,
        };

        let api_request: ApiRequest<CreateBadgeResponse> =
            ApiRequest::post(AdminApiV1::CreateBadge.path())
                .body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "创建勋章").await
    }
}

/// 创建勋章请求体
#[derive(Debug, Serialize)]
pub struct CreateBadgeBody {
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

/// 勋章多语言文案（i18n_name / i18n_explanation 的取值）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeI18nText {
    /// 中文文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 创建勋章响应
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 创建勋章的响应。
pub struct CreateBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 图标地址。
    pub icon_url: Option<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for CreateBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CreateBadgeRequestBuilder, will be removed in v1.0 (#271)")]
pub type CreateBadgeBuilder = CreateBadgeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateBadgeRequestBuilder::new(config.clone())
            .name("激励勋章".to_string())
            .explanation("这枚勋章为了激励员工颁发。".to_string())
            .detail_image("75a1949f-d9df-4b46-bc88-dacc51e88f3j".to_string())
            .show_image("03daa74a-159f-49e9-963e-b6c4d76103fj".to_string())
            .i18n_name(BadgeI18nText {
                zh_cn: Some("激励勋章".to_string()),
                en_us: Some("Incentive Medal".to_string()),
                ja_jp: None,
            });
        let _ = request;
    }

    /// 请求体序列化：字段与官方文档一致（name/detail_image/show_image 必填，
    /// explanation/i18n_* 可选取省略）。
    #[test]
    fn test_create_badge_body_serialization() {
        let body = CreateBadgeBody {
            name: "激励勋章".to_string(),
            explanation: None,
            detail_image: "75a1949f-d9df-4b46-bc88-dacc51e88f3j".to_string(),
            show_image: "03daa74a-159f-49e9-963e-b6c4d76103fj".to_string(),
            i18n_name: Some(BadgeI18nText {
                zh_cn: Some("激励勋章".to_string()),
                en_us: Some("Incentive Medal".to_string()),
                ja_jp: Some("奨励メダル".to_string()),
            }),
            i18n_explanation: None,
        };

        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["name"], "激励勋章");
        assert_eq!(
            value["detail_image"],
            "75a1949f-d9df-4b46-bc88-dacc51e88f3j"
        );
        assert_eq!(value["show_image"], "03daa74a-159f-49e9-963e-b6c4d76103fj");
        // 可选字段为 None 时不序列化
        assert!(value.get("explanation").is_none());
        assert!(value.get("i18n_explanation").is_none());
        assert_eq!(value["i18n_name"]["zh_cn"], "激励勋章");
    }
}
