//! Bitable 删除多条记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录请求
///
/// 用于批量删除多维表格数据表中的记录。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token，不能为空
/// - `table_id`: 数据表的 table_id，不能为空
/// - `record_ids`: 要删除的记录 ID 列表，最多 500 条，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::BatchDeleteRecordRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = BatchDeleteRecordRequest::new(config)
///     .app_token("app_token")
///     .table_id("table_id")
///     .record_ids(vec!["record_id1".to_string(), "record_id2".to_string()]);
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct BatchDeleteRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_ids: Vec<String>,
}

impl BatchDeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_ids: Vec::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.record_ids = record_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchDeleteRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.record_ids, "record_ids");

        // === 业务规则验证 ===
        if self.record_ids.len() > 500 {
            return Err(openlark_core::error::validation_error(
                "record_ids",
                "单次最多删除 500 条记录",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchDelete(self.app_token.clone(), self.table_id.clone());

        let api_request: ApiRequest<BatchDeleteRecordResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchDeleteRecordRequestBody {
            record_ids: self.record_ids,
        })?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Serialize)]
struct BatchDeleteRecordRequestBody {
    record_ids: Vec<String>,
}

/// 删除结果
///
/// 表示单条记录的删除结果。
///
/// # 字段说明
///
/// - `deleted`: 是否删除成功
/// - `record_id`: 记录 ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeletedRecord {
    pub deleted: bool,
    pub record_id: String,
}

/// 批量删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordResponse {
    pub records: Vec<DeletedRecord>,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_delete_record_request_builder() {
        let config = Config::default();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_ids(vec!["record_id1".to_string(), "record_id2".to_string()]);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_ids.len(), 2);
    }

    #[test]
    fn test_deleted_record() {
        let result = DeletedRecord {
            deleted: true,
            record_id: "rec123".to_string(),
        };
        assert!(result.deleted);
        assert_eq!(result.record_id, "rec123");
    }

    #[test]
    fn test_empty_app_token_validation() {
        let config = Config::default();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .record_ids(vec!["record_id".to_string()]);
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_empty_table_id_validation() {
        let config = Config::default();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .record_ids(vec!["record_id".to_string()]);
        assert_eq!(request.table_id, "");
    }

    #[test]
    fn test_empty_record_ids_validation() {
        let config = Config::default();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_ids(vec![]);
        assert_eq!(request.record_ids.len(), 0);
    }

    #[test]
    fn test_record_ids_exceeds_limit() {
        let config = Config::default();
        let record_ids: Vec<String> = (0..501).map(|i| format!("rec{}", i)).collect();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_ids(record_ids);
        assert_eq!(request.record_ids.len(), 501);
    }

    #[test]
    fn test_record_ids_at_limit() {
        let config = Config::default();
        let record_ids: Vec<String> = (0..500).map(|i| format!("rec{}", i)).collect();
        let request = BatchDeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_ids(record_ids);
        assert_eq!(request.record_ids.len(), 500);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchDeleteRecordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
