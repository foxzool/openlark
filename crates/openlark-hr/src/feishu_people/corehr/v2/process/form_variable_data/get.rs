//! 获取流程表单数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process.form_variable_data/get>

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
    /// 配置信息
    config: Config,
    /// 流程 ID
    process_id: Option<String>,
    body: Option<Value>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
            body: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = Some(process_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let process_id = self.process_id.unwrap_or_default();
        validate_required!(process_id.trim(), "process_id 不能为空");

        let mut request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/corehr/v2/processes/{process_id}/form_variable_data"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取流程表单数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 表单数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_variable_values: Option<Vec<FieldVariableValue>>,
    /// 流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

/// 表单变量（官方 field_variable_value 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableValue {
    /// 变量唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<FieldVariableI18nValue>,
    /// 变量值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FieldVariableValueTo>,
    /// 在 list_values 和 record_values 中引用的变量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_values: Option<Vec<FieldVariableSubValue>>,
}

/// 多语字段值（官方 field_variable_value_i18n 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableI18nValue {
    /// 中文值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 变量值（官方 field_variable_value_to 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableValueTo {
    /// 文本值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    /// 布尔值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    /// 数字值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    /// 枚举值，这里是枚举的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    /// 日期值，单位：天（从 1970-01-01 起经过的天数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<String>,
    /// 时间值，单位：ms（从 1970-01-01 起经过的毫秒数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    /// 多语字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FieldVariableI18nValue>,
    /// 对象值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FieldVariableObjectValue>,
    /// 用户 id，根据查询参数 user_id_type 选择对应的用户 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_value: Option<String>,
    /// 部门 id，根据查询参数 department_id_type 选择对应的部门 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<String>,
    /// 记录类型字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_values: Option<Vec<FieldVariableRecordValue>>,
    /// 员工类型字段值，为用户 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<String>,
    /// 数组类型值，每个元素都对应 sub_values 中的数组下标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_values: Option<Vec<String>>,
    /// 文件类型字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FieldVariableFileValue>,
}

/// 对象值（官方 field_variable_value_to_object 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableObjectValue {
    /// 飞书人事主数据对象唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// 飞书人事元数据对象的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

/// 记录类型字段值（官方 field_variable_value_to_record 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableRecordValue {
    /// 变量唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量值，对应 sub_values 中的 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_value_key: Option<String>,
    /// 记录唯一 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

/// 文件类型字段值（官方 field_variable_value_to_file 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableFileValue {
    /// 用于主数据文件下载 Open API 的入参 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_file_id: Option<String>,
    /// 文件名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小，单位：Byte
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    /// 文件类型，如 application/pdf
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// list/record 类型变量值引用（官方 field_variable_sub_vlaue 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableSubValue {
    /// 用于关联 list_values 和 record_values 类型变量值中的 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 变量值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldVariableValueTo>,
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

    /// 端到端：GET /open-apis/corehr/v2/processes/test001/form_variable_data
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v2/processes/test001/form_variable_data",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "field_variable_values": [
                        {
                            "variable_api_name": "custom123",
                            "variable_name": { "zh_cn": "北京", "en_us": "Beijing" },
                            "variable_value": {
                                "text_value": "测试测试",
                                "i18n_value": { "zh_cn": "北京", "en_us": "Beijing" },
                                "file_value": {
                                    "open_file_id": "66867ed00740ddd4a0bad4a5",
                                    "file_name": "file_name",
                                    "length": 65535,
                                    "mime_type": "application/pdf"
                                }
                            },
                            "sub_values": [
                                {
                                    "key": "key1",
                                    "value": { "user_value": "ou_test" }
                                }
                            ]
                        }
                    ],
                    "process_id": "7368832822481536556"
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

        assert_eq!(resp.process_id.as_deref(), Some("7368832822481536556"));
        let values = resp
            .field_variable_values
            .expect("field_variable_values 应解码");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].variable_api_name.as_deref(), Some("custom123"));
        assert_eq!(
            values[0]
                .variable_name
                .as_ref()
                .and_then(|v| v.zh_cn.as_deref()),
            Some("北京")
        );
        let value = values[0]
            .variable_value
            .as_ref()
            .expect("variable_value 应解码");
        assert_eq!(value.text_value.as_deref(), Some("测试测试"));
        assert_eq!(
            value
                .file_value
                .as_ref()
                .and_then(|f| f.mime_type.as_deref()),
            Some("application/pdf")
        );
        let sub_values = values[0].sub_values.as_ref().expect("sub_values 应解码");
        assert_eq!(sub_values[0].key.as_deref(), Some("key1"));
        assert_eq!(
            sub_values[0]
                .value
                .as_ref()
                .and_then(|v| v.user_value.as_deref()),
            Some("ou_test")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
