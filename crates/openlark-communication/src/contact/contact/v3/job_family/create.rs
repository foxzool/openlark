//! 创建序列
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_family::models::{I18nContent, JobFamilyResponse},
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 创建序列请求体
///
/// # 字段说明
///
/// - `name`: 序列名称，必填
/// - `description`: 序列描述，可选
/// - `parent_job_family_id`: 父序列 ID，可选
/// - `status`: 状态，必填
/// - `i18n_name`: 国际化名称，可选
/// - `i18n_description`: 国际化描述，可选
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateJobFamilyBody::new("技术序列", true);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobFamilyBody {
    /// 序列名称。
    name: String,
    /// 序列描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 父序列 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_job_family_id: Option<String>,
    /// 状态。
    status: bool,
    /// 国际化名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    i18n_name: Option<Vec<I18nContent>>,
    /// 国际化描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    i18n_description: Option<Vec<I18nContent>>,
}

impl CreateJobFamilyBody {
    /// 创建新的序列请求体。
    pub fn new(name: impl Into<String>, status: bool) -> Self {
        Self {
            name: name.into(),
            description: None,
            parent_job_family_id: None,
            status,
            i18n_name: None,
            i18n_description: None,
        }
    }

    /// 设置序列名称。
    /// 设置序列名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置序列描述。
    /// 设置序列描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置父序列 ID。
    /// 设置父序列 ID。
    pub fn parent_job_family_id(mut self, parent_job_family_id: impl Into<String>) -> Self {
        self.parent_job_family_id = Some(parent_job_family_id.into());
        self
    }

    /// 设置国际化名称。
    /// 设置国际化名称。
    pub fn i18n_name(mut self, i18n_name: Vec<I18nContent>) -> Self {
        self.i18n_name = Some(i18n_name);
        self
    }

    /// 设置国际化描述。
    /// 设置国际化描述。
    pub fn i18n_description(mut self, i18n_description: Vec<I18nContent>) -> Self {
        self.i18n_description = Some(i18n_description);
        self
    }
}

/// 创建序列请求
///
/// 用于创建新的职级序列。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateJobFamilyBody::new("技术序列", true);
/// let request = CreateJobFamilyRequest::new(config)
///     .execute(body).await?;
/// ```
pub struct CreateJobFamilyRequest {
    config: Config,
}

impl CreateJobFamilyRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/create>
    pub async fn execute(self, body: CreateJobFamilyBody) -> SDKResult<JobFamilyResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateJobFamilyBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobFamilyResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/job_families
        let req: ApiRequest<JobFamilyResponse> =
            ApiRequest::post(CONTACT_V3_JOB_FAMILIES).body(serialize_params(&body, "创建序列")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建序列")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_job_family_body_builder() {
        let body = CreateJobFamilyBody::new("技术序列", true);
        assert_eq!(body.name, "技术序列");
        assert!(body.status);
        assert!(body.description.is_none());
    }

    #[test]
    fn test_create_job_family_body_with_optional_fields() {
        let body = CreateJobFamilyBody::new("产品序列", true)
            .description("产品相关序列")
            .parent_job_family_id("parent_jf_xxx");
        assert_eq!(body.description, Some("产品相关序列".to_string()));
        assert_eq!(body.parent_job_family_id, Some("parent_jf_xxx".to_string()));
    }

    #[test]
    fn test_create_job_family_request_builder() {
        let config = Config::default();
        let request = CreateJobFamilyRequest::new(config);
        // Just verify the request can be created
        assert_eq!(request.config.app_id(), "");
    }

    #[test]
    fn test_create_job_family_body_chaining() {
        let body = CreateJobFamilyBody::new("测试序列", false)
            .name("新名称")
            .description("新描述");
        assert_eq!(body.name, "新名称");
        assert_eq!(body.description, Some("新描述".to_string()));
    }
}
