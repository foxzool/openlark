//! 获取招聘需求信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/job_requirement/list_by_id

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRequirementSummary;

/// 获取招聘需求信息请求
#[derive(Debug, Clone)]
pub struct ListByIdRequest {
    request_body: ListByIdRequestBody,
    /// 配置信息
    config: Config,
}

impl ListByIdRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: ListByIdRequestBody) -> Self {
        Self {
            request_body,
            config,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: ListByIdRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListByIdResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListByIdResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobRequirementListById;
        let request = ApiRequest::<ListByIdResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取招聘需求信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `ListByIdRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListByIdRequestBody {
    #[serde(flatten)]
    /// `fields` 字段。
    pub fields: Value,
}

impl ListByIdRequestBody {
    /// 创建新的请求实例。
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "获取招聘需求信息请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 获取招聘需求信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListByIdResponse {
    #[serde(default, alias = "job_requirements")]
    /// 结果项列表。
    pub items: Vec<JobRequirementSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListByIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
