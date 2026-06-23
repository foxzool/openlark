/// 更新块的内容
///
/// 更新指定块的内容。如果操作成功，接口将返回更新后的块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch
/// doc: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch
use crate::ccm::docx::models::block_update::BlockUpdateOperation;
use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 更新块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 更新操作（update_text_elements / insert_table_row 等 15 种之一）
    #[serde(flatten)]
    pub update: BlockUpdateOperation,
}

/// 更新块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockResponse {
    /// 更新后的块内容。
    pub block: DocxBlock,
    /// 文档版本号（操作后的文档版本）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_revision_id: Option<i32>,
    /// 幂等标记（请求时传入的 client_token 原样回传）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ApiResponseTrait for UpdateDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新块内容请求
///
/// 用于更新指定文档块的内容。
pub struct UpdateDocumentBlockRequest {
    config: Config,
}

impl UpdateDocumentBlockRequest {
    /// 创建新的更新块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: UpdateDocumentBlockParams,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: UpdateDocumentBlockParams,
        option: RequestOption,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::DocumentBlockPatch(params.document_id.clone(), params.block_id.clone());
        let mut api_request: ApiRequest<UpdateDocumentBlockResponse> =
            ApiRequest::patch(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新块的内容")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    use super::*;
    use crate::ccm::docx::models::block_update::{
        BlockUpdateOperation, InsertTableRowRequest, MergeTableCellsRequest,
    };

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

    #[test]
    fn test_block_update_operation_serializes_to_single_key() {
        // 官方期望 JSON：{"insert_table_row": {"row_index": 2}}
        let op = BlockUpdateOperation::InsertTableRow(InsertTableRowRequest { row_index: 2 });
        let json = serde_json::to_value(&op).unwrap();
        assert_eq!(json["insert_table_row"]["row_index"], 2);
        assert_eq!(json.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_block_update_operation_merge_cells_all_fields() {
        let op = BlockUpdateOperation::MergeTableCells(MergeTableCellsRequest {
            row_start_index: 0,
            row_end_index: 1,
            column_start_index: 0,
            column_end_index: 2,
        });
        let json = serde_json::to_value(&op).unwrap();
        let inner = &json["merge_table_cells"];
        assert_eq!(inner["row_start_index"], 0);
        assert_eq!(inner["column_end_index"], 2);
    }

    #[test]
    fn test_update_document_block_params_flatten_serialization() {
        // Params 通过 #[serde(flatten)] 把 update 操作平铺到顶层
        let params = UpdateDocumentBlockParams {
            document_id: "docx123".to_string(),
            block_id: "block456".to_string(),
            update: BlockUpdateOperation::InsertTableRow(InsertTableRowRequest { row_index: 1 }),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["insert_table_row"]["row_index"], 1);
        // 路径字段不参与序列化
        assert!(json.get("document_id").is_none());
    }
}
