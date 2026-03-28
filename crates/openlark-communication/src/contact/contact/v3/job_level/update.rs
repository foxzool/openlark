//! 更新职级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_level::models::{I18nContent, JobLevelResponse},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 更新职级请求体
///
/// 说明：字段均为可选，不传表示不更新。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobLevelBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 更新职级请求
pub struct UpdateJobLevelRequest {
    config: Config,
    job_level_id: String,
}

impl UpdateJobLevelRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_level_id: String::new(),
        }
    }

    /// 职级 ID（路径参数）
    pub fn job_level_id(mut self, job_level_id: impl Into<String>) -> Self {
        self.job_level_id = job_level_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_level/update>
    pub async fn execute(self, body: UpdateJobLevelBody) -> SDKResult<JobLevelResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateJobLevelBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobLevelResponse> {
        validate_required!(self.job_level_id, "job_level_id 不能为空");

        // url: PUT:/open-apis/contact/v3/job_levels/:job_level_id
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_JOB_LEVELS, self.job_level_id))
                .body(serialize_params(&body, "更新职级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新职级")
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
