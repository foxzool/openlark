//! 添加任务成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-add_members/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::TaskMember;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 添加任务成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddMembersBody {
    /// 成员列表
    pub members: Vec<TaskMember>,
    /// 幂等 token（长度 10~100 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

/// 添加任务成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddMembersResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 添加的成员列表
    #[serde(default)]
    pub members: Vec<TaskMember>,
}

/// 添加任务成员请求
#[derive(Debug, Clone)]
pub struct AddMembersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: AddMembersBody,
}

impl AddMembersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: AddMembersBody::default(),
        }
    }

    /// 设置成员列表
    pub fn members(mut self, members: Vec<TaskMember>) -> Self {
        self.body.members = members;
        self
    }

    /// 添加单个成员
    pub fn add_member(mut self, member: TaskMember) -> Self {
        self.body.members.push(member);
        self
    }

    /// 设置幂等 token
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.body.client_token = Some(client_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddMembersResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.members, "成员列表不能为空");

        let api_endpoint = TaskApiV2::TaskAddMembers(self.task_guid.clone());
        let mut request = ApiRequest::<AddMembersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "添加任务成员")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "添加任务成员",
        )
        .await
    }
}

impl ApiResponseTrait for AddMembersResponse {
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
    fn test_add_members_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddMembersRequest::new(config, "task_123")
            .members(vec![
                TaskMember {
                    id: "ou_user1".to_string(),
                    r#type: Some("user".to_string()),
                    role: Some("assignee".to_string()),
                    name: None,
                },
                TaskMember {
                    id: "ou_user2".to_string(),
                    r#type: Some("user".to_string()),
                    role: Some("follower".to_string()),
                    name: None,
                },
            ])
            .client_token("6d99f59c-4d7d-4452-98d6-3d0556393cf6");

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.members.len(), 2);
        assert_eq!(request.body.members[0].id, "ou_user1");
        assert_eq!(
            request.body.client_token.as_deref(),
            Some("6d99f59c-4d7d-4452-98d6-3d0556393cf6")
        );
    }

    #[test]
    fn test_add_member_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddMembersRequest::new(config, "task_123")
            .add_member(TaskMember {
                id: "ou_user1".to_string(),
                r#type: Some("user".to_string()),
                role: Some("assignee".to_string()),
                name: None,
            })
            .add_member(TaskMember {
                id: "ou_user2".to_string(),
                r#type: Some("user".to_string()),
                role: Some("assignee".to_string()),
                name: None,
            });

        assert_eq!(request.body.members.len(), 2);
        assert_eq!(request.body.members[1].id, "ou_user2");
    }

    #[test]
    fn test_add_members_body_serializes_member_objects() {
        let body = AddMembersBody {
            members: vec![TaskMember {
                id: "ou_user1".to_string(),
                r#type: Some("user".to_string()),
                role: Some("assignee".to_string()),
                name: Some("张明德".to_string()),
            }],
            client_token: None,
        };
        let json = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "members": [{"id": "ou_user1", "type": "user", "role": "assignee", "name": "张明德"}]
            })
        );
    }

    #[test]
    fn test_task_add_members_api_v2_url() {
        let endpoint = TaskApiV2::TaskAddMembers("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/add_members"
        );
    }
}
