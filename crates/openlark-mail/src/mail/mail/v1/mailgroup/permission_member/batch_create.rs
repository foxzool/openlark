//! 批量创建邮件组权限成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-permission_member/batch_create

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

/// Batch Create Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupPermissionMemberBody,
}

/// Batch Create Mail Group Permission Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupPermissionMemberBody {
    /// 成员列表。
    pub members: Vec<PermissionMemberItem>,
}

/// Permission Member Item。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberItem {
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Batch Create Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<BatchCreatePermissionMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Batch Create Permission Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberData {
    /// 结果列表。
    pub results: Vec<PermissionMemberResult>,
}

/// Permission Member Result。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberResult {
    /// 成员 ID。
    pub member_id: String,
    /// 状态。
    pub status: String,
}

impl BatchCreateMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupPermissionMemberBody::default(),
        }
    }

    /// members。
    pub fn members(mut self, members: Vec<PermissionMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupPermissionMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "请求")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组权限成员", "响应数据为空")
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
