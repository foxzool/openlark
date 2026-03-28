//! 添加任务依赖
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-add_dependencies/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 添加任务依赖请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddDependenciesBody {
    /// 依赖的任务 GUID 列表
    pub dependencies: Vec<String>,
}

/// 任务依赖信息
#[derive(Debug, Clone, Deserialize)]
pub struct TaskDependency {
    /// 依赖 GUID
    pub dependency_guid: String,
    /// 依赖的任务 GUID
    pub dependent_task_guid: String,
    /// 创建时间
    pub created_at: String,
}

/// 添加任务依赖响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddDependenciesResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 添加的依赖列表
    #[serde(default)]
    pub dependencies: Vec<TaskDependency>,
}

/// 添加任务依赖请求
#[derive(Debug, Clone)]
pub struct AddDependenciesRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: AddDependenciesBody,
}

impl AddDependenciesRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: AddDependenciesBody::default(),
        }
    }

    /// 设置依赖的任务 GUID 列表
    pub fn dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.body.dependencies = dependencies;
        self
    }

    /// 添加单个依赖任务
    pub fn add_dependency(mut self, dependency_task_guid: impl Into<String>) -> Self {
        self.body.dependencies.push(dependency_task_guid.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddDependenciesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddDependenciesResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.dependencies, "依赖任务列表不能为空");

        let api_endpoint = TaskApiV2::TaskAddDependencies(self.task_guid.clone());
        let mut request = ApiRequest::<AddDependenciesResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "添加任务依赖")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "添加任务依赖")
    }
}

impl ApiResponseTrait for AddDependenciesResponse {
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
    fn test_add_dependencies_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddDependenciesRequest::new(config, "task_123")
            .dependencies(vec!["task_456".to_string(), "task_789".to_string()]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.dependencies, vec!["task_456", "task_789"]);
    }

    #[test]
    fn test_add_dependency_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddDependenciesRequest::new(config, "task_123")
            .add_dependency("task_456")
            .add_dependency("task_789");

        assert_eq!(request.body.dependencies, vec!["task_456", "task_789"]);
    }

    #[test]
    fn test_task_add_dependencies_api_v2_url() {
        let endpoint = TaskApiV2::TaskAddDependencies("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/add_dependencies"
        );
    }
}
