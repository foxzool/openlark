//! 查询流程数据参数模板
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/query_flow_data_template/create>

use crate::feishu_people::corehr::v2::process_start::create::ProcessFormVariableV2;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询流程数据参数模板请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryFlowDataTemplateBody {
    /// 流程定义 ID
    pub flow_definition_id: String,
    /// 需要传入的变量的 ApiName，如果是多级下钻变量用 "." 分割（长度范围 1～1000）
    pub variable_api_names: Vec<String>,
}

/// 查询流程数据参数模板响应。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryFlowDataTemplateResponse {
    /// 流程参数模板
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values: Option<Vec<ProcessFormVariableV2>>,
    /// 错误信息（如变量 ApiName 无效、不可下钻等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
}

impl ApiResponseTrait for QueryFlowDataTemplateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询流程数据参数模板请求。
#[derive(Debug, Clone)]
pub struct QueryFlowDataTemplateRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 请求体。
    body: QueryFlowDataTemplateBody,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl QueryFlowDataTemplateRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryFlowDataTemplateBody::default(),
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: QueryFlowDataTemplateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置流程定义 ID。
    pub fn flow_definition_id(mut self, flow_definition_id: impl Into<String>) -> Self {
        self.body.flow_definition_id = flow_definition_id.into();
        self
    }

    /// 设置需要传入的变量的 ApiName 列表。
    pub fn variable_api_names(mut self, variable_api_names: Vec<String>) -> Self {
        self.body.variable_api_names = variable_api_names;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<QueryFlowDataTemplateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryFlowDataTemplateResponse> {
        validate_required!(self.body.flow_definition_id, "flow_definition_id 不能为空");
        validate_required_list!(
            self.body.variable_api_names,
            1000,
            "variable_api_names 不能为空且不能超过 1000 个"
        );

        let request = ApiRequest::<QueryFlowDataTemplateResponse>::post(
            "/open-apis/corehr/v2/query_flow_data_template",
        )
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "查询流程数据参数模板").await
    }
}
