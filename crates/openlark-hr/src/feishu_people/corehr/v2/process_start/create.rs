//! 发起流程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process_start/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 发起流程请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessStartBody {
    /// 流程定义 ID
    pub flow_definition_id: String,
    /// 发起人用户 ID，按 user_id_type 类型传递；如果 system_initiator 为 false，则必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 是否为系统身份发起流程；如果为 false，则 initiator_id 必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_initiator: Option<bool>,
    /// 业务数据（长度范围 0～100000）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_data: Option<Vec<ProcessFormVariableV2>>,
}

/// 发起流程响应。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessStartResponse {
    /// 流程运行实例 ID，详细信息可通过【获取单个流程详情】接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
}

impl ApiResponseTrait for ProcessStartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 流程表单变量（官方 process_form_variable_v2 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProcessFormVariableV2 {
    /// 变量唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_value: Option<FieldVariableValue>,
    /// 在 list_values 和 record_values 中引用的变量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_values: Option<Vec<FieldVariableSubValue>>,
}

/// 流程表单变量值（官方 field_variable_value_to_for_review 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableValue {
    /// 文本值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    /// 布尔值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    /// 数字值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    /// 枚举值，这里是枚举的 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    /// 从 1970 开始的天数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_value: Option<String>,
    /// 时间戳，毫秒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    /// 多语字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_value: Option<FieldVariableI18nValue>,
    /// 对象值，包括对象 id 和对象类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_value: Option<FieldVariableObjectValue>,
    /// 部门 id，根据入参选择对应的部门 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_value: Option<String>,
    /// 员工类型字段值，为用户 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_value: Option<String>,
    /// 数组类型值，里面包含多个值，每个元素都对应 sub_values 中的 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_values: Option<Vec<String>>,
    /// 文件类型字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_value: Option<FieldVariableFileValue>,
    /// record 类型字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_values: Option<Vec<FieldVariableRecordValue>>,
}

/// 多语字段值（官方 field_variable_value_i18n 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableI18nValue {
    /// 中文值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 对象值（官方 field_variable_value_to_object 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableObjectValue {
    /// wukong 的对象唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// wukong 的元数据唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wk_api_name: Option<String>,
}

/// 文件类型字段值（官方 field_variable_value_to_file_for_write 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableFileValue {
    /// 通过【上传文件】接口获得的 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_file_id: Option<String>,
    /// 文件名称（需带有文件后缀）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小，单位：Byte（取值范围 0～52428800）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

/// record 类型字段值（官方 field_variable_value_to_record 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableRecordValue {
    /// 变量唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_api_name: Option<String>,
    /// 变量值，对应 sub_values 中的 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_value_key: Option<String>,
    /// 记录唯一 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

/// list/record 类型变量值引用（官方 field_variable_sub_vlaue_for_review 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FieldVariableSubValue {
    /// 用于关联 list_values 和 record_values 类型变量值中的 key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 变量值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldVariableValue>,
}

/// 发起流程请求。
#[derive(Debug, Clone)]
pub struct ProcessStartRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 请求体。
    body: ProcessStartBody,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
    /// 部门 ID 类型。
    department_id_type: Option<String>,
}

impl ProcessStartRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ProcessStartBody::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: ProcessStartBody) -> Self {
        self.body = body;
        self
    }

    /// 设置流程定义 ID。
    pub fn flow_definition_id(mut self, flow_definition_id: impl Into<String>) -> Self {
        self.body.flow_definition_id = flow_definition_id.into();
        self
    }

    /// 设置发起人用户 ID（system_initiator 为 false 时必填）。
    pub fn initiator_id(mut self, initiator_id: impl Into<String>) -> Self {
        self.body.initiator_id = Some(initiator_id.into());
        self
    }

    /// 设置是否为系统身份发起流程。
    pub fn system_initiator(mut self, system_initiator: bool) -> Self {
        self.body.system_initiator = Some(system_initiator);
        self
    }

    /// 设置业务数据。
    pub fn flow_data(mut self, flow_data: Vec<ProcessFormVariableV2>) -> Self {
        self.body.flow_data = Some(flow_data);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ProcessStartResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProcessStartResponse> {
        validate_required!(self.body.flow_definition_id, "flow_definition_id 不能为空");

        let request =
            ApiRequest::<ProcessStartResponse>::post("/open-apis/corehr/v2/process_start")
                .query_opt("user_id_type", self.user_id_type)
                .query_opt("department_id_type", self.department_id_type)
                .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "发起流程").await
    }
}
