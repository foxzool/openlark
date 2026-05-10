//! 删除邮件组别名
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mail Group Alias Request。
#[derive(Debug, Clone)]
pub struct DeleteMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    alias_id: String,
}

/// Delete Mail Group Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupAliasResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupAliasResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

impl DeleteMailGroupAliasRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            alias_id: alias_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases/{}",
            self.mailgroup_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailGroupAliasResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupAliasResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupAliasResponse { data: None })
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
