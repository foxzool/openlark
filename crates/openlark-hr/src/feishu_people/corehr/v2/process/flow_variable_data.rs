//! 获取流程数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process/flow_variable_data>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取流程数据请求
#[derive(Debug, Clone)]
pub struct FlowVariableDataRequest {
    /// 配置信息
    config: Config,
    /// 流程 ID
    process_id: Option<String>,
    body: Option<Value>,
}

impl FlowVariableDataRequest {
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
    pub async fn execute(self) -> SDKResult<FlowVariableDataResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FlowVariableDataResponse> {
        let process_id = self.process_id.unwrap_or_default();
        validate_required!(process_id.trim(), "process_id 不能为空");

        let mut request = ApiRequest::<FlowVariableDataResponse>::get(format!(
            "/open-apis/corehr/v2/processes/{process_id}/flow_variable_data"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取流程数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowVariableDataResponse {
    /// 流程数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_variable_values: Option<Vec<FieldVariableValue>>,
    /// 流程实例 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

/// 流程字段变量（官方 field_variable_value 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableValue {
    /// 变量唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量名称（官方注明查询流程数据场景通常不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_name: Option<FieldVariableI18nValue>,
    /// 变量值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FieldVariableValueTo>,
    /// 在 list_values 和 record_values 中引用的变量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_values: Option<Vec<FieldVariableSubValue>>,
}

/// 流程变量值（官方 field_variable_value_to 结构）
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
    /// 从 1970 开始的天数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_value: Option<String>,
    /// 时间戳，毫秒
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    /// 多语字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FieldVariableI18nValue>,
    /// 对象值，包括对象 id 和对象类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FieldVariableObjectValue>,
    /// 用户 id，根据 user_type 选择对应的用户 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_value: Option<String>,
    /// 部门 id，根据入参选择对应的部门 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_value: Option<String>,
    /// 记录类型字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_values: Option<Vec<FieldVariableRecordValue>>,
    /// 员工类型字段值，为用户 id，根据入参选择返回的用户 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<String>,
    /// 数组类型值，里面包含多个值，每个元素都对应 sub_values 中的数组下标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_values: Option<Vec<String>>,
    /// 文件类型字段值，可通过主数据的文件下载 Open API 下载
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FieldVariableFileValue>,
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

/// 对象值（官方 field_variable_value_to_object 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableObjectValue {
    /// wukong 的对象唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// wukong 的元数据唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

/// 文件类型字段值（官方 field_variable_value_to_file 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableFileValue {
    /// 用于主数据文件下载接口的 id
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

impl ApiResponseTrait for FlowVariableDataResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/processes/test001/flow_variable_data
    #[tokio::test]
    async fn test_flow_variable_data_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v2/processes/test001/flow_variable_data",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "field_variable_values": [
                        {
                            "variable_api_name": "custom123",
                            "variable_name": {
                                "zh_cn": "北京",
                                "en_us": "Beijing"
                            },
                            "variable_value": {
                                "text_value": "测试测试",
                                "bool_value": true,
                                "user_value": "ou_c57053dad6eea0aea4696c48433d8562",
                                "list_values": ["key1"],
                                "file_value": {
                                    "open_file_id": "66867ed00740ddd4a0bad4a5_c99b5322dc744fe4b99b76426ffe5d53",
                                    "file_name": "file_name",
                                    "length": 65535,
                                    "mime_type": "application/pdf"
                                }
                            },
                            "sub_values": [
                                {
                                    "key": "key1",
                                    "value": {
                                        "text_value": "测试测试"
                                    }
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

        let resp = FlowVariableDataRequest::new(config)
            .process_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.process_id, Some("7368832822481536556".to_string()));
        let values = resp
            .field_variable_values
            .expect("field_variable_values 应存在");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].variable_api_name, Some("custom123".to_string()));
        assert_eq!(
            values[0].variable_name,
            Some(FieldVariableI18nValue {
                zh_cn: Some("北京".to_string()),
                en_us: Some("Beijing".to_string()),
            })
        );
        let variable_value = values[0]
            .variable_value
            .clone()
            .expect("variable_value 应存在");
        assert_eq!(
            variable_value.user_value,
            Some("ou_c57053dad6eea0aea4696c48433d8562".to_string())
        );
        assert_eq!(
            variable_value.file_value,
            Some(FieldVariableFileValue {
                open_file_id: Some(
                    "66867ed00740ddd4a0bad4a5_c99b5322dc744fe4b99b76426ffe5d53".to_string()
                ),
                file_name: Some("file_name".to_string()),
                length: Some(65535),
                mime_type: Some("application/pdf".to_string()),
            })
        );
        let sub_values = values[0].sub_values.clone().expect("sub_values 应存在");
        assert_eq!(sub_values.len(), 1);
        assert_eq!(sub_values[0].key, Some("key1".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
