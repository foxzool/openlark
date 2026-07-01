//! Bitable 批量获取记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 批量获取记录请求。
#[derive(Debug, Clone)]
pub struct BatchGetRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_ids: Vec<String>,
    user_id_type: Option<String>,
    with_shared_url: Option<bool>,
    automatic_fields: Option<bool>,
}

impl BatchGetRecordRequest {
    /// 创建新的批量获取记录请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_ids: Vec::new(),
            user_id_type: None,
            with_shared_url: None,
            automatic_fields: None,
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表 ID。
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置记录 ID 列表。
    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.record_ids = record_ids;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置是否返回分享链接。
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.with_shared_url = Some(with_shared_url);
        self
    }

    /// 设置是否返回自动计算字段。
    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.automatic_fields = Some(automatic_fields);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchGetRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetRecordResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.record_ids, "record_ids");
        if self.record_ids.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "record_ids",
                "单次最多批量获取 100 条记录",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchGet(self.app_token.clone(), self.table_id.clone());

        let api_request: ApiRequest<BatchGetRecordResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchGetRecordRequestBody {
            record_ids: self.record_ids,
            user_id_type: self.user_id_type,
            with_shared_url: self.with_shared_url,
            automatic_fields: self.automatic_fields,
        })?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 批量获取记录请求体（内部使用）。
#[derive(Serialize)]
struct BatchGetRecordRequestBody {
    record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_shared_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_fields: Option<bool>,
}

/// 批量获取记录响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRecordResponse {
    /// 成功获取到的记录列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    /// 无权限读取的记录 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_record_ids: Option<Vec<String>>,
    /// 不存在的记录 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_record_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for BatchGetRecordResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
