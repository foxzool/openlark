//! 获取流程表单数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/process-form-variable-data/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取流程表单数据请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    process_id: String,
}

impl GetRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: String::new(),
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = process_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.process_id.trim(), "process_id 不能为空");

        let api_endpoint = CorehrApiV1::ProcessFormVariableDataGet(self.process_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(_option),
            "获取流程表单数据响应为空",
        )
        .await
    }
}

/// 获取流程表单数据响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 流程变量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_variable_values: Option<Vec<FormFieldVariable>>,
    /// 流程 id（仅出现在官方响应示例中，字段表未列出，按示例建模）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 流程表单变量
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariable {
    /// 变量 api 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量名称的 i18n 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<BpmDataengineI18n>,
    /// 变量值的对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FormVariableValueInfo>,
}

/// BPM dataengine i18n 文本
///
/// 官方文档字段表展示为 `zh_cn` / `en_us`，但实际返回的 JSON Key 为
/// `zh-CN` / `en-US`（文档兼容性注释明示），此处两种键名均兼容。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BpmDataengineI18n {
    /// i18n 类型字段，中文值
    #[serde(
        rename = "zh-CN",
        alias = "zh_cn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zh_cn: Option<String>,
    /// i18n 类型字段，英文值
    #[serde(
        rename = "en-US",
        alias = "en_us",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub en_us: Option<String>,
}

/// 变量值的对象
///
/// 按变量类型条件返回，同一时刻仅一个 `*_value` 变体有值。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormVariableValueInfo {
    /// 文本变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<FormFieldVariableTextValue>,
    /// 数值变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<FormFieldVariableNumberValue>,
    /// 日期变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<FormFieldVariableDateValue>,
    /// 员工变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<FormFieldVariableEmploymentValue>,
    /// 日期时间变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<FormFieldVariableDatetimeValue>,
    /// 枚举变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<FormFieldVariableEnumValue>,
    /// 空变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<FormFieldVariableNullValue>,
    /// 布尔变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<FormFieldVariableBoolValue>,
    /// 部门变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<FormFieldVariableDepartmentValue>,
    /// 文件变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FormFieldVariableFileValue>,
    /// i18n 变量对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FormFieldVariableI18nValue>,
    /// 对象变量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FormFieldVariableObjectValue>,
    /// 列表对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_value: Option<FormFieldVariableListValue>,
    /// 记录对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_value: Option<FormFieldVariableRecordValue>,
}

/// 文本变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableTextValue {
    /// 文本类型变量的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 数值变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableNumberValue {
    /// 数值类型变量的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 日期变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableDateValue {
    /// 日期变量的值，从 1970 起的天数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// 员工变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableEmploymentValue {
    /// employmentID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 日期时间变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableDatetimeValue {
    /// 毫秒的时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// 时区，如 +08:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// 枚举变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableEnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 枚举的名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<BpmDataengineI18n>,
    /// 枚举的描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<BpmDataengineI18n>,
    /// 枚举常量集 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_source_id: Option<i64>,
    /// 枚举常量集类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_source_type: Option<i32>,
    /// 主数据 apiName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

/// 空变量对象（无字段）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableNullValue {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 布尔变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableBoolValue {
    /// 布尔变量的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// 部门变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableDepartmentValue {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 文件变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableFileValue {
    /// 文件源类型（1 BPM / 2 主数据）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<i32>,
    /// 文件 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 文件名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件长度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    /// 扩展类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// i18n 变量对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableI18nValue {
    /// i18n 值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BpmDataengineI18n>,
}

/// 对象变量
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableObjectValue {
    /// 对象 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 主数据 apiName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

/// 列表对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableListValue {
    /// 列表值（元素结构与变量值对象一致）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<FormVariableValueInfo>>,
}

/// 记录对象
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FormFieldVariableRecordValue {
    /// 记录值：Key 是变量唯一标识，Value 是变量值（结构与变量值对象一致）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::HashMap<String, FormVariableValueInfo>>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v1/processes/test001/form_variable_data
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v1/processes/test001/form_variable_data",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "field_variable_values": [
                        {
                            "variable_api_name": "reason",
                            "variable_name": {"en-US": "Reason", "zh-CN": "离职原因"},
                            "variable_value": {
                                "enum_value": {
                                    "desc": null,
                                    "enum_source_id": 123,
                                    "enum_source_type": 2,
                                    "name": {"en-US": "", "zh-CN": "测试"},
                                    "value": "reason_for_offboarding_option96",
                                    "wk_api_name": "reason_for_offboarding"
                                }
                            }
                        },
                        {
                            "variable_api_name": "offboarding_date",
                            "variable_name": {"en-US": "Offboarding date", "zh-CN": "离职日期"},
                            "variable_value": {
                                "date_value": {"value": 19265}
                            }
                        },
                        {
                            "variable_api_name": "employment",
                            "variable_name": {"en-US": "Employment", "zh-CN": "员工"},
                            "variable_value": {
                                "employment_value": {"user_id": "123321", "value": "123321"}
                            }
                        }
                    ],
                    "process_id": "123321"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetRequest::new(config)
            .process_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.process_id.as_deref(), Some("123321"));
        let values = resp
            .field_variable_values
            .expect("field_variable_values 应解析出流程变量");
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].variable_api_name.as_deref(), Some("reason"));
        assert_eq!(
            values[0]
                .variable_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("离职原因")
        );
        let enum_value = values[0]
            .variable_value
            .as_ref()
            .and_then(|v| v.enum_value.as_ref())
            .expect("enum_value 应解析");
        assert_eq!(
            enum_value.value.as_deref(),
            Some("reason_for_offboarding_option96")
        );
        assert_eq!(enum_value.enum_source_id, Some(123));
        assert_eq!(
            values[1]
                .variable_value
                .as_ref()
                .and_then(|v| v.date_value.as_ref())
                .and_then(|d| d.value),
            Some(19265)
        );
        assert_eq!(
            values[2]
                .variable_value
                .as_ref()
                .and_then(|v| v.employment_value.as_ref())
                .and_then(|e| e.user_id.as_deref()),
            Some("123321")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
