//! 移除任务依赖
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-remove_dependencies/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除任务依赖请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemoveDependenciesBody {
    /// 依赖 GUID 列表
    pub dependency_guids: Vec<String>,
}

/// 移除任务依赖响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveDependenciesResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 移除的依赖 GUID 列表
    #[serde(default)]
    pub removed_dependencies: Vec<String>,
}

/// 移除任务依赖请求
#[derive(Debug, Clone)]
pub struct RemoveDependenciesRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: RemoveDependenciesBody,
}

impl RemoveDependenciesRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: RemoveDependenciesBody::default(),
        }
    }

    /// 设置依赖 GUID 列表
    pub fn dependency_guids(mut self, dependency_guids: Vec<String>) -> Self {
        self.body.dependency_guids = dependency_guids;
        self
    }

    /// 移除单个依赖
    pub fn remove_dependency(mut self, dependency_guid: impl Into<String>) -> Self {
        self.body.dependency_guids.push(dependency_guid.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveDependenciesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveDependenciesResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.dependency_guids, "依赖GUID列表不能为空");

        let api_endpoint = TaskApiV2::TaskRemoveDependencies(self.task_guid.clone());
        let mut request = ApiRequest::<RemoveDependenciesResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "移除任务依赖")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "移除任务依赖")
    }
}

impl ApiResponseTrait for RemoveDependenciesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_remove_dependencies_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveDependenciesRequest::new(config, "task_123")
            .dependency_guids(vec!["dep_1".to_string(), "dep_2".to_string()]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.dependency_guids, vec!["dep_1", "dep_2"]);
    }

    #[test]
    fn test_remove_dependency_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveDependenciesRequest::new(config, "task_123")
            .remove_dependency("dep_1")
            .remove_dependency("dep_2");

        assert_eq!(request.body.dependency_guids, vec!["dep_1", "dep_2"]);
    }

    #[test]
    fn test_task_remove_dependencies_api_v2_url() {
        let endpoint = TaskApiV2::TaskRemoveDependencies("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/remove_dependencies"
        );
    }
}
