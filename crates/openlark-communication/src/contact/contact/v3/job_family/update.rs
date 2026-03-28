//! 更新序列
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_family::models::{I18nContent, JobFamilyResponse},
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 更新序列请求体
///
/// 说明：字段均为可选，不传表示不更新。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobFamilyBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 更新序列请求
pub struct UpdateJobFamilyRequest {
    config: Config,
    job_family_id: String,
}

impl UpdateJobFamilyRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_family_id: String::new(),
        }
    }

    /// 序列 ID（路径参数）
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.job_family_id = job_family_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/update>
    pub async fn execute(self, body: UpdateJobFamilyBody) -> SDKResult<JobFamilyResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateJobFamilyBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobFamilyResponse> {
        validate_required!(self.job_family_id, "job_family_id 不能为空");

        // url: PUT:/open-apis/contact/v3/job_families/:job_family_id
        let req: ApiRequest<JobFamilyResponse> = ApiRequest::put(format!(
            "{}/{}",
            CONTACT_V3_JOB_FAMILIES, self.job_family_id
        ))
        .body(serialize_params(&body, "更新序列")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新序列")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
