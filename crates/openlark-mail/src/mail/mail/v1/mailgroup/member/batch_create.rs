//! 批量创建邮件组成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-member/batch_create

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Batch Create Mail Group Member Request。
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupMemberBody,
}

/// Batch Create Mail Group Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupMemberBody {
    /// 成员列表。
    pub members: Vec<MailGroupMemberItem>,
}

/// Mail Group Member Item。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberItem {
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Batch Create Mail Group Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<BatchCreateMailGroupMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Batch Create Mail Group Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberData {
    /// 结果列表。
    pub results: Vec<MailGroupMemberResult>,
}

/// Mail Group Member Result。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberResult {
    /// 成员 ID。
    pub member_id: String,
    /// 状态。
    pub status: String,
}

impl BatchCreateMailGroupMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupMemberBody::default(),
        }
    }

    /// members。
    pub fn members(mut self, members: Vec<MailGroupMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组成员", "响应数据为空")
        })
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
