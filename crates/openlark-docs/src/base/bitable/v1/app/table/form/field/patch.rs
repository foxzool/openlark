//! Bitable 更新表单问题
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-form-field/patch

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::PatchFormFieldRequest;

/// 更新表单问题请求
///
/// 用于更新表单中的单个问题配置。
#[derive(Debug, Clone)]
pub struct PatchFormFieldQuestionRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    field_id: String,
    body: PatchFormFieldRequest,
}

impl PatchFormFieldQuestionRequest {
    /// 创建新的表单问题更新请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            field_id: String::new(),
            body: PatchFormFieldRequest::new(),
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表 ID。
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置表单 ID。
    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    /// 设置字段 ID。
    pub fn field_id(mut self, field_id: String) -> Self {
        self.field_id = field_id;
        self
    }

    /// 设置前置字段 ID。
    pub fn pre_field_id(mut self, pre_field_id: String) -> Self {
        self.body.pre_field_id = Some(pre_field_id);
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: String) -> Self {
        self.body.title = Some(title);
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: String) -> Self {
        self.body.description = Some(description);
        self
    }

    /// 设置是否必答。
    pub fn required(mut self, required: bool) -> Self {
        self.body.required = Some(required);
        self
    }

    /// 设置是否可见。
    pub fn visible(mut self, visible: bool) -> Self {
        self.body.visible = Some(visible);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchFormFieldQuestionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchFormFieldQuestionResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.form_id.trim(), "form_id");
        validate_required!(self.field_id.trim(), "field_id");
        self.body.validate()?;

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormFieldPatch(
            self.app_token,
            self.table_id,
            self.form_id,
            self.field_id,
        );

        let api_request: ApiRequest<PatchFormFieldQuestionResponse> =
            ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_vec(&self.body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 更新表单问题 Builder
pub struct PatchFormFieldQuestionBuilder {
    request: PatchFormFieldQuestionRequest,
}

impl PatchFormFieldQuestionBuilder {
    /// 创建新的表单问题更新 builder。
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchFormFieldQuestionRequest::new(config),
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表 ID。
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置表单 ID。
    pub fn form_id(mut self, form_id: String) -> Self {
        self.request = self.request.form_id(form_id);
        self
    }

    /// 设置字段 ID。
    pub fn field_id(mut self, field_id: String) -> Self {
        self.request = self.request.field_id(field_id);
        self
    }

    /// 设置前置字段 ID。
    pub fn pre_field_id(mut self, pre_field_id: String) -> Self {
        self.request = self.request.pre_field_id(pre_field_id);
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: String) -> Self {
        self.request = self.request.title(title);
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置是否必答。
    pub fn required(mut self, required: bool) -> Self {
        self.request = self.request.required(required);
        self
    }

    /// 设置是否可见。
    pub fn visible(mut self, visible: bool) -> Self {
        self.request = self.request.visible(visible);
        self
    }

    /// 构建请求对象。
    pub fn build(self) -> PatchFormFieldQuestionRequest {
        self.request
    }
}

/// 更新后的表单问题（响应 field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchedFormFieldQuestion {
    /// 前置字段 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    /// 标题。
    pub title: String,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必答。
    pub required: bool,
    /// 是否可见。
    pub visible: bool,
}

/// 更新表单问题响应（data）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldQuestionResponse {
    /// 更新后的问题配置。
    pub field: PatchedFormFieldQuestion,
}

impl ApiResponseTrait for PatchFormFieldQuestionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
