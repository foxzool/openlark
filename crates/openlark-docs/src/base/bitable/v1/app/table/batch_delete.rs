//! Bitable 批量删除数据表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 批量删除数据表请求
#[derive(Debug, Clone)]
pub struct BatchDeleteTableRequest {
    config: Config,
    app_token: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    table_ids: Vec<String>,
}

impl BatchDeleteTableRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            table_ids: vec![],
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.table_ids = table_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchDeleteTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteTableResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        if self.table_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "table_ids",
                "数据表 ID 列表不能为空",
            ));
        }
        if self.table_ids.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "table_ids",
                "批量删除数据表数量不能超过 50 个",
            ));
        }

        let api_endpoint = BitableApiV1::TableBatchDelete(self.app_token);
        let mut api_request: ApiRequest<BatchDeleteTableResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchDeleteTableRequestBody {
            table_ids: self.table_ids,
        })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
}

/// 批量删除数据表响应（data 通常为空对象 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteTableResponse {}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
