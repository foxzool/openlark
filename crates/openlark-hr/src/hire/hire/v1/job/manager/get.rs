//! 获取职位上的招聘人员信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job.manager/get>

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
pub struct GetRequest {
    job_id: String,
    manager_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self {
            job_id,
            manager_id: String::new(),
            config,
        }
    }

    /// 设置 `manager_id`。
    pub fn manager_id(mut self, manager_id: String) -> Self {
        self.manager_id = manager_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        validate_required!(self.job_id.trim(), "职位 ID 不能为空");
        validate_required!(self.manager_id.trim(), "招聘人员 ID 不能为空");

        let request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/hire/v1/jobs/{}/managers/{}",
            self.job_id, self.manager_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位上的招聘人员信息响应数据为空",
        )
        .await
    }
}

/// 获取职位上的招聘人员信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `info` 字段（官方响应 data 下的职位负责人容器）。
    pub info: Option<JobManagerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager` 字段。
    pub manager: Option<JobRecruiterRecord>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位负责人（官方 `job.manager` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘负责人 ID，仅一位。
    pub recruiter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用人经理 ID 列表。
    pub hiring_manager_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 协助人 ID 列表。
    pub assistant_id_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/jobs/test001/managers/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/jobs/test001/managers/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "info": {
                        "id": "1618209327096",
                        "recruiter_id": "ou_efk39117c300506837def50545420c6a",
                        "hiring_manager_id_list": ["on_94a1ee5551019f18cd73d9f111898cf2"],
                        "assistant_id_list": ["on_94a1ee5551019f18cd73d9f111898cf2"]
                    }
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

        let resp = GetRequest::new(config, "test001".to_string())
            .manager_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let info = resp.info.as_ref().expect("info 容器应存在");
        assert_eq!(info.id.as_deref(), Some("1618209327096"));
        assert_eq!(
            info.recruiter_id.as_deref(),
            Some("ou_efk39117c300506837def50545420c6a")
        );
        assert_eq!(
            info.hiring_manager_id_list.as_ref().map(|v| v.len()),
            Some(1)
        );
        assert_eq!(info.assistant_id_list.as_ref().map(|v| v.len()), Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
