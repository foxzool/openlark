//! 获取单个序列信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_family::models::JobFamilyResponse,
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 获取单个序列信息请求
///
/// 用于获取指定职级序列的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `job_family_id`: 序列 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetJobFamilyRequest::new(config)
///     .job_family_id("jf_xxx")
///     .execute().await?;
/// ```
pub struct GetJobFamilyRequest {
    config: Config,
    job_family_id: String,
}

impl GetJobFamilyRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/job_family/get>
    pub async fn execute(self) -> SDKResult<JobFamilyResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<JobFamilyResponse> {
        // === 必填字段验证 ===
        validate_required!(self.job_family_id, "job_family_id 不能为空");

        // url: GET:/open-apis/contact/v3/job_families/:job_family_id
        let req: ApiRequest<JobFamilyResponse> = ApiRequest::get(format!(
            "{}/{}",
            CONTACT_V3_JOB_FAMILIES, self.job_family_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单个序列信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_job_family_request_builder() {
        let config = Config::default();
        let request = GetJobFamilyRequest::new(config).job_family_id("jf_xxx");
        assert_eq!(request.job_family_id, "jf_xxx");
    }

    #[test]
    fn test_get_job_family_request_default_values() {
        let config = Config::default();
        let request = GetJobFamilyRequest::new(config);
        assert_eq!(request.job_family_id, "");
    }

    #[test]
    fn test_get_job_family_request_with_different_ids() {
        let config = Config::default();
        let request1 = GetJobFamilyRequest::new(config.clone()).job_family_id("jf_111");
        let request2 = GetJobFamilyRequest::new(config).job_family_id("jf_222");
        assert_eq!(request1.job_family_id, "jf_111");
        assert_eq!(request2.job_family_id, "jf_222");
    }
}
