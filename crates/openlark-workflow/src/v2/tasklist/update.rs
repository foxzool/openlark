//! 更新清单
//!
//! docPath: <https://open.feishu.cn/document/task-v2/tasklist/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::models::{
    TasklistMemberDetail, UpdateTasklistBody, UpdateTasklistResponse,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新清单请求
#[derive(Debug, Clone)]
pub struct UpdateTasklistRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: UpdateTasklistBody,
}

impl UpdateTasklistRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, tasklist_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            body: UpdateTasklistBody::default(),
        }
    }

    /// 将字段名加入 update_fields（去重）
    fn push_update_field(&mut self, field: &str) {
        if !self.body.update_fields.iter().any(|f| f == field) {
            self.body.update_fields.push(field.to_string());
        }
    }

    /// 设置清单名称，并将 name 加入 update_fields
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.tasklist.name = Some(name.into());
        self.push_update_field("name");
        self
    }

    /// 设置清单所有者，并将 owner 加入 update_fields
    pub fn owner(mut self, owner: TasklistMemberDetail) -> Self {
        self.body.tasklist.owner = Some(owner);
        self.push_update_field("owner");
        self
    }

    /// 显式设置要更新的字段名列表（覆盖 Builder 自动收集的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 设置更新所有者后原所有者的新角色（editor / viewer / none，默认 none）
    pub fn origin_owner_to_role(mut self, origin_owner_to_role: impl Into<String>) -> Self {
        self.body.origin_owner_to_role = Some(origin_owner_to_role.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTasklistResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.update_fields, "update_fields 不能为空");

        let api_endpoint = TaskApiV2::TasklistUpdate(self.tasklist_guid.clone());
        let mut request = ApiRequest::<UpdateTasklistResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新清单")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新清单",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateTasklistResponse {
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
    fn test_update_tasklist_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateTasklistRequest::new(config, "tasklist_123".to_string())
            .name("年会工作任务清单")
            .owner(TasklistMemberDetail {
                id: Some("ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f".to_string()),
                r#type: Some("user".to_string()),
                role: Some("owner".to_string()),
                name: Some("张明德（明德）".to_string()),
            })
            .origin_owner_to_role("editor");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(
            request.body.tasklist.name,
            Some("年会工作任务清单".to_string())
        );
        assert_eq!(
            request
                .body
                .tasklist
                .owner
                .as_ref()
                .and_then(|o| o.id.as_deref()),
            Some("ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f")
        );
        assert_eq!(request.body.update_fields, vec!["name", "owner"]);
        assert_eq!(
            request.body.origin_owner_to_role,
            Some("editor".to_string())
        );
    }

    #[test]
    fn test_update_tasklist_body_serialization() {
        let body = UpdateTasklistBody {
            tasklist: crate::v2::tasklist::models::InputTasklist {
                name: Some("年会工作任务清单".to_string()),
                owner: Some(TasklistMemberDetail {
                    id: Some("ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f".to_string()),
                    r#type: Some("user".to_string()),
                    role: Some("owner".to_string()),
                    name: Some("张明德（明德）".to_string()),
                }),
            },
            update_fields: vec!["owner".to_string()],
            origin_owner_to_role: Some("editor".to_string()),
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "tasklist": {
                    "name": "年会工作任务清单",
                    "owner": {
                        "id": "ou_2cefb2f014f8d0c6c2d2eb7bafb0e54f",
                        "type": "user",
                        "role": "owner",
                        "name": "张明德（明德）"
                    }
                },
                "update_fields": ["owner"],
                "origin_owner_to_role": "editor"
            })
        );
    }
}
