//! 更新自定义字段选项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field-option/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::option::create::CustomFieldOption;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 要更新的自定义字段选项数据（input_option）
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct InputCustomFieldOption {
    /// 选项名称（最长 50 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 颜色索引值（0~54）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    /// 将当前选项放在某选项之前（传 option_guid）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    /// 将当前选项放在某选项之后（传 option_guid）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    /// 是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

/// 更新自定义字段选项请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldOptionBody {
    /// 要更新的 option 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<InputCustomFieldOption>,
    /// 要更新的字段名（支持 name/color_index/is_hidden/insert_before/insert_after，1~20 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

/// 更新自定义字段选项响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomFieldOptionResponse {
    /// 选项信息
    pub option: CustomFieldOption,
}

/// 更新自定义字段选项请求
#[derive(Debug, Clone)]
pub struct UpdateCustomFieldOptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 自定义字段 GUID
    custom_field_guid: String,
    /// 选项 GUID
    option_guid: String,
    /// 请求体
    body: UpdateCustomFieldOptionBody,
}

impl UpdateCustomFieldOptionRequest {
    /// 创建新的请求构建器。
    pub fn new(
        config: Arc<Config>,
        custom_field_guid: impl Into<String>,
        option_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            option_guid: option_guid.into(),
            body: UpdateCustomFieldOptionBody::default(),
        }
    }

    /// 设置选项名称
    pub fn name(self, name: impl Into<String>) -> Self {
        self.with_option_field("name", |option| option.name = Some(name.into()))
    }

    /// 设置颜色索引值（0~54）
    pub fn color_index(self, color_index: i32) -> Self {
        self.with_option_field("color_index", |option| {
            option.color_index = Some(color_index)
        })
    }

    /// 将当前选项放在某选项之前（传 option_guid）
    pub fn insert_before(self, insert_before: impl Into<String>) -> Self {
        self.with_option_field("insert_before", |option| {
            option.insert_before = Some(insert_before.into())
        })
    }

    /// 将当前选项放在某选项之后（传 option_guid）
    pub fn insert_after(self, insert_after: impl Into<String>) -> Self {
        self.with_option_field("insert_after", |option| {
            option.insert_after = Some(insert_after.into())
        })
    }

    /// 设置是否隐藏
    pub fn is_hidden(self, is_hidden: bool) -> Self {
        self.with_option_field("is_hidden", |option| option.is_hidden = Some(is_hidden))
    }

    /// 设置要更新的字段名（覆盖 builder 自动记录的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = Some(update_fields);
        self
    }

    /// 写入嵌套 option 字段并自动记录到 update_fields。
    fn with_option_field(
        mut self,
        field_name: &str,
        apply: impl FnOnce(&mut InputCustomFieldOption),
    ) -> Self {
        let option = self
            .body
            .option
            .get_or_insert_with(InputCustomFieldOption::default);
        apply(option);
        let fields = self.body.update_fields.get_or_insert_with(Vec::new);
        if !fields.iter().any(|f| f == field_name) {
            fields.push(field_name.to_string());
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateCustomFieldOptionResponse> {
        // 验证必填字段
        validate_required!(self.custom_field_guid.trim(), "自定义字段GUID不能为空");
        validate_required!(self.option_guid.trim(), "选项GUID不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionUpdate(
            self.custom_field_guid.clone(),
            self.option_guid.clone(),
        );
        let mut request =
            ApiRequest::<UpdateCustomFieldOptionResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新自定义字段选项")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新自定义字段选项",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateCustomFieldOptionResponse {
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
    fn test_update_custom_field_option_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateCustomFieldOptionRequest::new(config, "custom_field_123", "option_456")
            .name("高优")
            .color_index(10);

        assert_eq!(request.custom_field_guid, "custom_field_123");
        assert_eq!(request.option_guid, "option_456");
        assert_eq!(
            request.body.option.as_ref().and_then(|o| o.name.as_deref()),
            Some("高优")
        );
        assert_eq!(
            request.body.update_fields.as_deref(),
            Some(["name".to_string(), "color_index".to_string()].as_slice())
        );
    }

    #[test]
    fn test_update_custom_field_option_body_official_shape() {
        // 官方请求示例：{"option": {...}, "update_fields": ["name"]}
        let body = UpdateCustomFieldOptionBody {
            option: Some(InputCustomFieldOption {
                name: Some("高优".to_string()),
                color_index: Some(10),
                insert_before: None,
                insert_after: None,
                is_hidden: Some(false),
            }),
            update_fields: Some(vec!["name".to_string()]),
        };
        let json = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "option": {
                    "name": "高优",
                    "color_index": 10,
                    "is_hidden": false
                },
                "update_fields": ["name"]
            })
        );
    }

    #[test]
    fn test_update_custom_field_option_empty_body_skips_fields() {
        // 未设置任何字段时不序列化 option/update_fields
        let body = UpdateCustomFieldOptionBody::default();
        let json = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn test_custom_field_option_update_api_v2_url() {
        let endpoint = TaskApiV2::CustomFieldOptionUpdate(
            "custom_field_123".to_string(),
            "option_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/custom_fields/custom_field_123/options/option_456"
        );
    }
}
