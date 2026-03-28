//! 移除任务成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-remove_members/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除任务成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemoveMembersBody {
    /// 成员 ID 列表
    pub members: Vec<String>,
}

/// 移除任务成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveMembersResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 移除的成员 ID 列表
    #[serde(default)]
    pub removed_members: Vec<String>,
}

/// 移除任务成员请求
#[derive(Debug, Clone)]
pub struct RemoveMembersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: RemoveMembersBody,
}

impl RemoveMembersRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: RemoveMembersBody::default(),
        }
    }

    /// 设置成员 ID 列表
    pub fn members(mut self, members: Vec<String>) -> Self {
        self.body.members = members;
        self
    }

    /// 移除单个成员
    pub fn remove_member(mut self, member: impl Into<String>) -> Self {
        self.body.members.push(member.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveMembersResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.members, "成员列表不能为空");

        let api_endpoint = TaskApiV2::TaskRemoveMembers(self.task_guid.clone());
        let mut request = ApiRequest::<RemoveMembersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "移除任务成员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "移除任务成员")
    }
}

impl ApiResponseTrait for RemoveMembersResponse {
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
    fn test_remove_members_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveMembersRequest::new(config, "task_123")
            .members(vec!["user_1".to_string(), "user_2".to_string()]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_remove_member_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveMembersRequest::new(config, "task_123")
            .remove_member("user_1")
            .remove_member("user_2");

        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_task_remove_members_api_v2_url() {
        let endpoint = TaskApiV2::TaskRemoveMembers("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/remove_members"
        );
    }
}
