//! 获取职位上的招聘人员信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/recruiter>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRecruiterRecord;

/// 获取职位上的招聘人员信息请求
#[derive(Debug, Clone)]
pub struct RecruiterRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl RecruiterRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self { job_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecruiterResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RecruiterResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobRecruiter(self.job_id);
        let request = ApiRequest::<RecruiterResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位上的招聘人员信息响应数据为空",
        )
        .await
    }
}

/// 职位招聘人员信息（官方 `job_recruiter2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruiterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘负责人 ID，与 `user_id_type` 查询参数类型一致。
    pub recruiter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用人经理 ID 列表，与 `user_id_type` 查询参数类型一致。
    pub hiring_manager_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 协助人 ID 列表，与 `user_id_type` 查询参数类型一致。
    pub assistant_id_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取职位上的招聘人员信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecruiterResponse {
    #[serde(default, alias = "items", alias = "recruiters")]
    /// `recruiters` 字段。
    pub recruiters: Vec<JobRecruiterRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位招聘人员信息（官方 `data.info`）。
    pub info: Option<JobRecruiterInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for RecruiterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_recruiter_request_builder_new() {
        let request = RecruiterRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }

    /// 官方响应示例：`data.info` 应被类型化解析。
    #[test]
    fn test_recruiter_response_parses_official_shape() {
        let resp: RecruiterResponse = serde_json::from_value(serde_json::json!({
            "info": {
                "id": "1618209327096",
                "recruiter_id": "ou_efk39117c300506837def50545420c6a",
                "hiring_manager_id_list": ["on_94a1ee5551019f18cd73f9f111898cf2"],
                "assistant_id_list": ["on_94a1ee5551019f18cd73f9f111898cf2"]
            }
        }))
        .expect("官方示例应能反序列化");

        let info = resp.info.as_ref().expect("info 应被类型化解析");
        assert_eq!(info.id.as_deref(), Some("1618209327096"));
        assert_eq!(
            info.recruiter_id.as_deref(),
            Some("ou_efk39117c300506837def50545420c6a")
        );
        assert_eq!(
            info.hiring_manager_id_list.as_deref(),
            Some(&["on_94a1ee5551019f18cd73f9f111898cf2".to_string()][..])
        );
        assert_eq!(
            info.assistant_id_list.as_deref(),
            Some(&["on_94a1ee5551019f18cd73f9f111898cf2".to_string()][..])
        );
    }
}
