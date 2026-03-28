//! 创建数据知识
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSETS};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建数据知识请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataAssetBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateDataAssetBody {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            tags: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

/// 创建数据知识请求
///
/// 用于创建新的数据知识。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 请求体字段
///
/// - `name`: 名称，必填
/// - `description`: 描述（可选）
/// - `tags`: 标签列表（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateDataAssetBody::new("知识库")
///     .description("产品文档")
///     .tags(vec!["文档".to_string()]);
/// let request = CreateDataAssetRequest::new(config)
///     .app_id("app_xxx")
///     .execute(body).await?;
/// ```
pub struct CreateDataAssetRequest {
    config: Config,
    app_id: String,
}

impl CreateDataAssetRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create>
    pub async fn execute(self, body: CreateDataAssetBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateDataAssetBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_DATA_ASSETS.replace("{app_id}", &self.app_id);
        let req: ApiRequest<CreateDataAssetBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建数据知识")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_data_asset_request_builder() {
        let config = Config::default();
        let request = CreateDataAssetRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_create_data_asset_body_builder() {
        let body = CreateDataAssetBody::new("知识库");
        assert_eq!(body.name, "知识库");
        assert_eq!(body.description, None);
    }

    #[test]
    fn test_create_data_asset_body_with_all_fields() {
        let body = CreateDataAssetBody::new("知识库")
            .description("产品文档")
            .tags(vec!["文档".to_string(), "产品".to_string()]);
        assert_eq!(body.name, "知识库");
        assert_eq!(body.description, Some("产品文档".to_string()));
        assert_eq!(body.tags.as_ref().map(|v| v.len()), Some(2));
    }

    #[test]
    fn test_create_data_asset_request_default_values() {
        let config = Config::default();
        let request = CreateDataAssetRequest::new(config);
        assert_eq!(request.app_id, "");
    }

    #[test]
    fn test_create_data_asset_body_with_tags_only() {
        let body = CreateDataAssetBody::new("知识库").tags(vec!["标签1".to_string()]);
        assert!(body.tags.is_some());
        assert_eq!(body.description, None);
    }
}
