//! 创建分组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::section::models::{CreateSectionBody, CreateSectionResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建分组请求
#[derive(Debug, Clone)]
pub struct CreateSectionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateSectionBody,
}

impl CreateSectionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateSectionBody {
                // 官方默认资源类型为 tasklist（清单）
                resource_type: "tasklist".to_string(),
                ..CreateSectionBody::default()
            },
        }
    }

    /// 设置自定义分组名（必填，最大 100 个 UTF-8 字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置资源类型（必填，"tasklist" 清单 / "my_tasks" 我负责的）
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.body.resource_type = resource_type.into();
        self
    }

    /// 设置归属资源 ID（resource_type 为 "tasklist" 时填写清单 GUID）
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.body.resource_id = Some(resource_id.into());
        self
    }

    /// 插入到该分组之前的分组 GUID（与 insert_after 互斥）
    pub fn insert_before(mut self, insert_before: impl Into<String>) -> Self {
        self.body.insert_before = Some(insert_before.into());
        self
    }

    /// 插入到该分组之后的分组 GUID（与 insert_before 互斥）
    pub fn insert_after(mut self, insert_after: impl Into<String>) -> Self {
        self.body.insert_after = Some(insert_after.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateSectionResponse> {
        // 验证必填字段
        validate_required!(self.body.name.trim(), "自定义分组名不能为空");
        validate_required!(self.body.resource_type.trim(), "资源类型不能为空");

        let api_endpoint = TaskApiV2::SectionCreate;
        let mut request = ApiRequest::<CreateSectionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建分组")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建分组",
        )
        .await
    }
}

impl ApiResponseTrait for CreateSectionResponse {
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
    fn test_create_section_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateSectionRequest::new(config)
            .name("测试分组")
            .resource_id("tasklist_123");

        assert_eq!(request.body.name, "测试分组");
        assert_eq!(request.body.resource_type, "tasklist");
        assert_eq!(request.body.resource_id.as_deref(), Some("tasklist_123"));

        // 必填字段序列化包含 name / resource_type
        let json = serde_json::to_value(&request.body).unwrap();
        assert_eq!(json["name"], "测试分组");
        assert_eq!(json["resource_type"], "tasklist");
        assert_eq!(json["resource_id"], "tasklist_123");
    }
}
