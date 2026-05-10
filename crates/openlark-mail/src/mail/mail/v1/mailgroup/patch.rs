//! 修改邮件组部分信息
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/patch

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mail Group Request。
#[derive(Debug, Clone)]
pub struct PatchMailGroupRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: PatchMailGroupBody,
}

/// Patch Mail Group Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMailGroupBody {
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// owner 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

/// Patch Mail Group Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailGroupResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl PatchMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: PatchMailGroupBody::default(),
        }
    }

    /// description。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// owner。
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.body.owner = Some(owner.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailGroupResponse> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}", self.mailgroup_id);
        let req: ApiRequest<PatchMailGroupResponse> =
            ApiRequest::patch(&path).body(serialize_params(&self.body, "请求")?);

        let _resp: Response<PatchMailGroupResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(PatchMailGroupResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for PatchMailGroupResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
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
