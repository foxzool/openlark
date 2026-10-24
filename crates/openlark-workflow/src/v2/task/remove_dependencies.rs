//! 移除任务依赖
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-remove_dependencies/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 要移除的任务依赖（task_dependency）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskDependencyInput {
    /// 依赖任务的 GUID
    pub task_guid: String,
}

/// 移除任务依赖请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemoveDependenciesBody {
    /// 要移除的依赖列表（1~50 个）
    pub dependencies: Vec<TaskDependencyInput>,
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
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: RemoveDependenciesBody::default(),
        }
    }

    /// 设置要移除的依赖列表
    pub fn dependencies(mut self, dependencies: Vec<TaskDependencyInput>) -> Self {
        self.body.dependencies = dependencies;
        self
    }

    /// 移除单个依赖（传依赖任务的 GUID）
    pub fn remove_dependency(mut self, task_guid: impl Into<String>) -> Self {
        self.body.dependencies.push(TaskDependencyInput {
            task_guid: task_guid.into(),
        });
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
        validate_required_list!(
            self.body.dependencies,
            50,
            "依赖列表不能为空且不能超过 50 个"
        );

        let api_endpoint = TaskApiV2::TaskRemoveDependencies(self.task_guid.clone());
        let mut request = ApiRequest::<RemoveDependenciesResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "移除任务依赖")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "移除任务依赖",
        )
        .await
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

        let request = RemoveDependenciesRequest::new(config, "task_123").dependencies(vec![
            TaskDependencyInput {
                task_guid: "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0".to_string(),
            },
            TaskDependencyInput {
                task_guid: "83912691-2e43-47fc-94a4-d512e03984fa".to_string(),
            },
        ]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.dependencies.len(), 2);
        assert_eq!(
            request.body.dependencies[0].task_guid,
            "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"
        );
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
            .remove_dependency("93b7bd05-35e6-4371-b3c9-6b7cbd7100c0")
            .remove_dependency("83912691-2e43-47fc-94a4-d512e03984fa");

        assert_eq!(request.body.dependencies.len(), 2);
        assert_eq!(
            request.body.dependencies[1].task_guid,
            "83912691-2e43-47fc-94a4-d512e03984fa"
        );
    }

    #[test]
    fn test_remove_dependencies_body_official_shape() {
        // 官方请求示例：{"dependencies": [{"task_guid": "..."}]}
        let body = RemoveDependenciesBody {
            dependencies: vec![TaskDependencyInput {
                task_guid: "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0".to_string(),
            }],
        };
        let json = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "dependencies": [{"task_guid": "93b7bd05-35e6-4371-b3c9-6b7cbd7100c0"}]
            })
        );
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
