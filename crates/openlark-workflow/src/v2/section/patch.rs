//! 更新自定义分组（PATCH）
//!
//! docPath: <https://open.feishu.cn/document/task-v2/section/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::section::models::{UpdateSectionBody, UpdateSectionResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新分组请求
#[derive(Debug, Clone)]
pub struct UpdateSectionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 分组 GUID
    section_guid: String,
    /// 请求体
    body: UpdateSectionBody,
    /// 用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    user_id_type: Option<String>,
}

impl UpdateSectionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, section_guid: String) -> Self {
        Self {
            config,
            section_guid,
            body: UpdateSectionBody::default(),
            user_id_type: None,
        }
    }

    /// 将字段名加入 update_fields（去重）
    fn push_update_field(&mut self, field: &str) {
        if !self.body.update_fields.iter().any(|f| f == field) {
            self.body.update_fields.push(field.to_string());
        }
    }

    /// 设置分组名，并将 name 加入 update_fields
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.section.name = Some(name.into());
        self.push_update_field("name");
        self
    }

    /// 插入到该分组之前的分组 GUID，并将 insert_before 加入 update_fields
    pub fn insert_before(mut self, insert_before: impl Into<String>) -> Self {
        self.body.section.insert_before = Some(insert_before.into());
        self.push_update_field("insert_before");
        self
    }

    /// 插入到该分组之后的分组 GUID，并将 insert_after 加入 update_fields
    pub fn insert_after(mut self, insert_after: impl Into<String>) -> Self {
        self.body.section.insert_after = Some(insert_after.into());
        self.push_update_field("insert_after");
        self
    }

    /// 显式设置要更新的字段名列表（覆盖 Builder 自动收集的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateSectionResponse> {
        // 验证必填字段
        validate_required!(self.section_guid.trim(), "分组GUID不能为空");
        validate_required!(self.body.update_fields, "update_fields 不能为空");

        let api_endpoint = TaskApiV2::SectionUpdate(self.section_guid.clone());
        let mut request = ApiRequest::<UpdateSectionResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新分组")?);
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新分组",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateSectionResponse {
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
    fn test_update_section_builder_collects_update_fields() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateSectionRequest::new(config, "section_456".to_string())
            .name("更新的标题")
            .user_id_type("open_id");

        assert_eq!(request.section_guid, "section_456");
        assert_eq!(request.body.section.name, Some("更新的标题".to_string()));
        assert_eq!(request.body.update_fields, vec!["name"]);
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    #[test]
    fn test_update_section_body_serialization() {
        let body = UpdateSectionBody {
            section: crate::v2::section::models::InputSection {
                name: Some("已经审核过的任务".to_string()),
                insert_before: Some("e6e37dcc-f75a-5936-f589-12fb4b5c80c2".to_string()),
                ..Default::default()
            },
            update_fields: vec!["name".to_string()],
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "section": {
                    "name": "已经审核过的任务",
                    "insert_before": "e6e37dcc-f75a-5936-f589-12fb4b5c80c2"
                },
                "update_fields": ["name"]
            })
        );
    }
}
