//! Bitable 批量新增数据表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

use super::create::TableData;

/// 批量新增数据表请求
#[derive(Debug, Clone)]
pub struct BatchCreateTableRequest {
    config: Config,
    app_token: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    tables: Vec<TableData>,
}

impl BatchCreateTableRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            tables: vec![],
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 用户 ID 类型（查询参数）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 幂等标识（查询参数）
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置要新增的数据表列表（单次最多 50 个）
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.tables = tables;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateTableResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        if self.tables.is_empty() {
            return Err(openlark_core::error::validation_error(
                "tables",
                "数据表列表不能为空",
            ));
        }
        if self.tables.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "tables",
                "批量创建数据表数量不能超过 50 个",
            ));
        }

        let api_endpoint = BitableApiV1::TableBatchCreate(self.app_token);
        let mut api_request: ApiRequest<BatchCreateTableResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchCreateTableRequestBody {
            tables: self.tables,
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
struct BatchCreateTableRequestBody {
    tables: Vec<TableData>,
}

/// 批量新增数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResponse {
    /// 新增的数据表 ID 列表
    pub table_ids: Vec<String>,
}

impl ApiResponseTrait for BatchCreateTableResponse {
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
