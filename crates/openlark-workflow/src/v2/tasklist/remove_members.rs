//! 移除清单成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-remove_members/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除清单成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemoveTasklistMembersBody {
    /// 成员 ID 列表
    pub members: Vec<String>,
}

/// 移除清单成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveTasklistMembersResponse {
    /// 任务清单 GUID
    pub tasklist_guid: String,
    /// 移除的成员 ID 列表
    #[serde(default)]
    pub removed_members: Vec<String>,
}

/// 移除清单成员请求
#[derive(Debug, Clone)]
pub struct RemoveTasklistMembersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: RemoveTasklistMembersBody,
}

impl RemoveTasklistMembersRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            body: RemoveTasklistMembersBody::default(),
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
    pub async fn execute(self) -> SDKResult<RemoveTasklistMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveTasklistMembersResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.members, "成员列表不能为空");

        let api_endpoint = TaskApiV2::TasklistRemoveMembers(self.tasklist_guid.clone());
        let mut request = ApiRequest::<RemoveTasklistMembersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "移除清单成员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "移除清单成员")
    }
}

impl ApiResponseTrait for RemoveTasklistMembersResponse {
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
    fn test_remove_tasklist_members_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveTasklistMembersRequest::new(config, "tasklist_123")
            .members(vec!["user_1".to_string(), "user_2".to_string()]);

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_remove_tasklist_member_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveTasklistMembersRequest::new(config, "tasklist_123")
            .remove_member("user_1")
            .remove_member("user_2");

        assert_eq!(request.body.members, vec!["user_1", "user_2"]);
    }

    #[test]
    fn test_tasklist_remove_members_api_v2_url() {
        let endpoint = TaskApiV2::TasklistRemoveMembers("tasklist_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/remove_members"
        );
    }
}
