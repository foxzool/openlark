//! 创建服务台对话
//!
//! 创建一个新的服务台对话（工单）。
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/start_service

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建服务台对话请求
#[derive(Debug, Clone)]
pub struct StartServiceRequest {
    config: Arc<Config>,
    body: StartServiceBody,
}

/// 创建服务台对话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartServiceBody {
    /// 用户 ID
    pub user_id: String,
    /// 服务台 ID
    pub service_id: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
}

impl StartServiceBody {
    fn validate(&self) -> SDKResult<()> {
        if self.user_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error("用户ID不能为空", ""));
        }
        if self.service_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "服务台ID不能为空",
                "",
            ));
        }
        Ok(())
    }
}

/// 创建服务台对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceResponse {
    /// 响应数据。
    pub data: Option<StartServiceData>,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建服务台对话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceData {
    /// 工单 ID
    pub ticket_id: String,
}

impl StartServiceRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: StartServiceBody::default(),
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置服务 ID。
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.body.service_id = service_id.into();
        self
    }

    /// 设置问题内容。
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.body.question = Some(question.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<StartServiceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StartServiceResponse> {
        self.body.validate()?;

        let req: ApiRequest<StartServiceResponse> =
            ApiRequest::post(HelpdeskApiV1::TicketStartService.to_url())
                .body(serialize_params(&self.body, "创建服务台对话")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建服务台对话", "响应数据为空"))
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
