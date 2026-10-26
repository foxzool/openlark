//! 创建自定义字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::models::{
    CreateCustomFieldBody, CreateCustomFieldResponse, CustomFieldType, DatetimeSetting,
    MemberSetting, NumberSetting, SelectSetting, TextSetting,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建自定义字段请求
#[derive(Debug, Clone)]
pub struct CreateCustomFieldRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateCustomFieldBody,
}

impl CreateCustomFieldRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, resource_id: String) -> Self {
        Self {
            config,
            body: CreateCustomFieldBody {
                resource_type: "tasklist".to_string(),
                resource_id,
                ..CreateCustomFieldBody::default()
            },
        }
    }

    /// 设置字段名称（必填，最大 50 个字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置字段类型（必填：number / datetime / member / single_select / multi_select / text）
    pub fn field_type(mut self, field_type: CustomFieldType) -> Self {
        self.body.field_type = field_type;
        self
    }

    /// 设置数字类型字段设置
    pub fn number_setting(mut self, setting: NumberSetting) -> Self {
        self.body.number_setting = Some(setting);
        self
    }

    /// 设置人员类型字段设置
    pub fn member_setting(mut self, setting: MemberSetting) -> Self {
        self.body.member_setting = Some(setting);
        self
    }

    /// 设置日期时间类型字段设置
    pub fn datetime_setting(mut self, setting: DatetimeSetting) -> Self {
        self.body.datetime_setting = Some(setting);
        self
    }

    /// 设置单选类型字段设置
    pub fn single_select_setting(mut self, setting: SelectSetting) -> Self {
        self.body.single_select_setting = Some(setting);
        self
    }

    /// 设置多选类型字段设置
    pub fn multi_select_setting(mut self, setting: SelectSetting) -> Self {
        self.body.multi_select_setting = Some(setting);
        self
    }

    /// 设置文本类型字段设置
    pub fn text_setting(mut self, setting: TextSetting) -> Self {
        self.body.text_setting = Some(setting);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldResponse> {
        // 验证必填字段
        validate_required!(self.body.resource_id.trim(), "清单GUID不能为空");
        validate_required!(self.body.name.trim(), "字段名称不能为空");

        let api_endpoint = TaskApiV2::CustomFieldCreate;
        let mut request = ApiRequest::<CreateCustomFieldResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建自定义字段")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建自定义字段",
        )
        .await
    }
}

impl ApiResponseTrait for CreateCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    use crate::v2::custom_field::models::{CustomFieldType, SelectOption};

    #[test]
    fn test_create_custom_field_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateCustomFieldRequest::new(config, "tasklist_123".to_string())
            .name("优先级")
            .field_type(CustomFieldType::SingleSelect)
            .single_select_setting(SelectSetting {
                options: Some(vec![
                    SelectOption {
                        name: "高优".to_string(),
                        color_index: Some(1),
                        is_hidden: Some(false),
                    },
                    SelectOption {
                        name: "低优".to_string(),
                        ..Default::default()
                    },
                ]),
            });

        assert_eq!(request.body.name, "优先级");
        assert_eq!(request.body.field_type, CustomFieldType::SingleSelect);

        // 必填字段以官方形状序列化：type 位于顶层，无 config 包装
        let json = serde_json::to_value(&request.body).unwrap();
        assert_eq!(json["name"], "优先级");
        assert_eq!(json["type"], "single_select");
        assert_eq!(json["resource_type"], "tasklist");
        assert_eq!(json["resource_id"], "tasklist_123");
        assert!(json.get("config").is_none());
        assert_eq!(
            json["single_select_setting"]["options"][0]["name"],
            "高优".to_string()
        );
    }
}
