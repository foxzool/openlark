//! 添加清单成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-add_members/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 添加清单成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddTasklistMembersBody {
    /// 成员 ID 列表
    pub members: Vec<String>,
}

/// 清单成员信息
#[derive(Debug, Clone, Deserialize)]
pub struct TasklistMember {
    /// 成员 ID
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 添加时间
    pub added_at: String,
}

/// 添加清单成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddTasklistMembersResponse {
    /// 任务清单 GUID
    pub tasklist_guid: String,
    /// 添加的成员列表
    #[serde(default)]
    pub members: Vec<TasklistMember>,
}

/// 添加清单成员请求
#[derive(Debug, Clone)]
pub struct AddTasklistMembersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: AddTasklistMembersBody,
}

impl AddTasklistMembersRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            body: AddTasklistMembersBody::default(),
        }
    }

    /// 设置成员 ID 列表
    pub fn members(mut self, members: Vec<String>) -> Self {
        self.body.members = members;
        self
    }

    /// 添加单个成员
    pub fn add_member(mut self, member: impl Into<String>) -> Self {
        self.body.members.push(member.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddTasklistMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddTasklistMembersResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.members, "成员列表不能为空");

        let api_endpoint = TaskApiV2::TasklistAddMembers(self.tasklist_guid.clone());
        let mut request = ApiRequest::<AddTasklistMembersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "添加清单成员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "添加清单成员")
    }
}

impl ApiResponseTrait for AddTasklistMembersResponse {
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
    fn test_add_tasklist_members_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddTasklistMembersRequest::new(config, "tasklist_123")
            .members(vec!["user_1".to_string(), "user_2".to_string()]);

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_add_tasklist_member_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddTasklistMembersRequest::new(config, "tasklist_123")
            .add_member("user_1")
            .add_member("user_2");

        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_tasklist_add_members_api_v2_url() {
        let endpoint = TaskApiV2::TasklistAddMembers("tasklist_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/add_members"
        );
    }
}
