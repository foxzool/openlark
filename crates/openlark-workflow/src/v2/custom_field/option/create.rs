//! 创建自定义字段选项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field-option/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建自定义字段选项请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCustomFieldOptionBody {
    /// 选项名称（最长 50 字符）
    pub name: String,
    /// 颜色索引值（0~54，不填则随机选择）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    /// 将当前选项放在某选项之前（传 option_guid）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    /// 将当前选项放在某选项之后（传 option_guid）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    /// 是否隐藏（默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

/// 自定义字段选项信息
#[derive(Debug, Clone, Deserialize)]
pub struct CustomFieldOption {
    /// 选项 GUID
    #[serde(default)]
    pub guid: String,
    /// 选项名称
    #[serde(default)]
    pub name: String,
    /// 颜色索引值（0~54）
    #[serde(default)]
    pub color_index: Option<i32>,
    /// 是否隐藏
    #[serde(default)]
    pub is_hidden: Option<bool>,
}

/// 创建自定义字段选项响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomFieldOptionResponse {
    /// 选项信息
    pub option: CustomFieldOption,
}

/// 创建自定义字段选项请求
#[derive(Debug, Clone)]
pub struct CreateCustomFieldOptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 自定义字段 GUID
    custom_field_guid: String,
    /// 请求体
    body: CreateCustomFieldOptionBody,
}

impl CreateCustomFieldOptionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: CreateCustomFieldOptionBody::default(),
        }
    }

    /// 设置选项名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置颜色索引值（0~54）
    pub fn color_index(mut self, color_index: i32) -> Self {
        self.body.color_index = Some(color_index);
        self
    }

    /// 将当前选项放在某选项之前（传 option_guid）
    pub fn insert_before(mut self, insert_before: impl Into<String>) -> Self {
        self.body.insert_before = Some(insert_before.into());
        self
    }

    /// 将当前选项放在某选项之后（传 option_guid）
    pub fn insert_after(mut self, insert_after: impl Into<String>) -> Self {
        self.body.insert_after = Some(insert_after.into());
        self
    }

    /// 设置是否隐藏
    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.body.is_hidden = Some(is_hidden);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldOptionResponse> {
        // 验证必填字段
        validate_required!(self.custom_field_guid.trim(), "自定义字段GUID不能为空");
        validate_required!(self.body.name.trim(), "选项名称不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionCreate(self.custom_field_guid.clone());
        let mut request =
            ApiRequest::<CreateCustomFieldOptionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建自定义字段选项")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建自定义字段选项",
        )
        .await
    }
}

impl ApiResponseTrait for CreateCustomFieldOptionResponse {
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
    fn test_create_custom_field_option_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateCustomFieldOptionRequest::new(config, "custom_field_123")
            .name("高优")
            .color_index(10)
            .insert_before("2bd905f8-ef38-408b-aa1f-2b2ad33b2913")
            .insert_after("b13adf3c-cad6-4e02-8929-550c112b5633")
            .is_hidden(false);

        assert_eq!(request.custom_field_guid, "custom_field_123");
        assert_eq!(request.body.name, "高优");
        assert_eq!(request.body.color_index, Some(10));
        assert_eq!(
            request.body.insert_before.as_deref(),
            Some("2bd905f8-ef38-408b-aa1f-2b2ad33b2913")
        );

        // 官方请求示例形状：仅序列化已设置的字段
        let json = serde_json::to_value(&request.body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "name": "高优",
                "color_index": 10,
                "insert_before": "2bd905f8-ef38-408b-aa1f-2b2ad33b2913",
                "insert_after": "b13adf3c-cad6-4e02-8929-550c112b5633",
                "is_hidden": false
            })
        );
    }

    #[test]
    fn test_custom_field_option_create_api_v2_url() {
        let endpoint = TaskApiV2::CustomFieldOptionCreate("custom_field_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/custom_fields/custom_field_123/options"
        );
    }
}
