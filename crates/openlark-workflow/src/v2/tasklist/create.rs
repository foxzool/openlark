//! 创建任务清单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::models::{
    CreateTasklistBody, CreateTasklistResponse, TasklistMemberDetail,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建任务清单请求
#[derive(Debug, Clone)]
pub struct CreateTasklistRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateTasklistBody,
}

impl CreateTasklistRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTasklistBody::default(),
        }
    }

    /// 设置清单名称（必填，最多 100 个字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置清单成员列表（可选，最多 500 个）
    pub fn members(mut self, members: Vec<TasklistMemberDetail>) -> Self {
        self.body.members = Some(members);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTasklistResponse> {
        // 验证必填字段
        validate_required!(self.body.name.trim(), "清单名称不能为空");

        let api_endpoint = TaskApiV2::TasklistCreate;
        let mut request = ApiRequest::<CreateTasklistResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建任务清单")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建任务清单",
        )
        .await
    }
}

impl ApiResponseTrait for CreateTasklistResponse {
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
    fn test_create_tasklist_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTasklistRequest::new(config)
            .name("测试任务清单")
            .members(vec![TasklistMemberDetail {
                id: Some("ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f".to_string()),
                r#type: Some("user".to_string()),
                role: Some("editor".to_string()),
                name: Some("张明德".to_string()),
            }]);

        assert_eq!(request.body.name, "测试任务清单");
        let members = request.body.members.expect("members 应已设置");
        assert_eq!(members.len(), 1);
        assert_eq!(
            members[0].id.as_deref(),
            Some("ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f")
        );
        assert_eq!(members[0].role.as_deref(), Some("editor"));
    }
}
