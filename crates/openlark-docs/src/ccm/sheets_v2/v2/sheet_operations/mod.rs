/// CCM Sheet V2 表格操作API 模块
///
/// 表格操作API实现，包含行列操作、合并单元格等功能：
/// - delete_range: 删除范围
/// - insert_dimension: 插入行列
/// - move_dimension: 移动行列
/// - replace_range: 替换范围
/// - find_replace: 查找替换
/// - merge_cells: 合并单元格
/// - unmerge_cells: 取消合并单元格
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 表格操作API结构体
#[derive(Debug, Clone)]
pub struct SheetOperationsApi {
    config: Config,
}

impl SheetOperationsApi {
    /// 创建新的表格操作API实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出模型定义
/// models 子模块。
pub mod models;
// models 模块显式导出
/// 重新导出相关类型。
pub use models::{
    DeleteRangeParams, DeleteRangeResponse, DeleteRangeResult, FindReplaceParams,
    FindReplaceResponse, FindReplaceResult, InsertDimensionParams, InsertDimensionResponse,
    InsertDimensionResult, MergeCellsParams, MergeCellsResponse, MergeCellsResult,
    MoveDimensionParams, MoveDimensionResponse, MoveDimensionResult, ReplaceRangeParams,
    ReplaceRangeResponse, ReplaceRangeResult, UnmergeCellsParams, UnmergeCellsResponse,
    UnmergeCellsResult,
};

impl ApiResponseTrait for DeleteRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for InsertDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for MoveDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ReplaceRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FindReplaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UnmergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除范围
///
/// 根据 spreadsheetToken 和 range 删除指定范围的行列。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/deleting-ranges
pub async fn delete_range(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteRangeParams,
) -> SDKResult<DeleteRangeResponse> {
    delete_range_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 删除范围（带选项）
///
/// 根据 spreadsheetToken 和 range 删除指定范围的行列。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/deleting-ranges
pub async fn delete_range_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: DeleteRangeParams,
    option: RequestOption,
) -> SDKResult<DeleteRangeResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "删除范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteRange(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteRangeResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "删除范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除范围")
}

/// 插入行列
///
/// 根据 spreadsheetToken 和 range 在指定位置插入空行或空列。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/inserting-blank-rows-or-columns
pub async fn insert_dimension(
    config: &Config,
    spreadsheet_token: &str,
    params: InsertDimensionParams,
) -> SDKResult<InsertDimensionResponse> {
    insert_dimension_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 插入行列（带选项）
///
/// 根据 spreadsheetToken 和 range 在指定位置插入空行或空列。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/inserting-blank-rows-or-columns
pub async fn insert_dimension_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: InsertDimensionParams,
    option: RequestOption,
) -> SDKResult<InsertDimensionResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "插入范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::InsertDimension(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<InsertDimensionResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "插入行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "插入行列")
}

/// 移动行列
///
/// 根据 spreadsheetToken 将指定范围的行或列移动到新位置。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/moving-rows-or-columns
pub async fn move_dimension(
    config: &Config,
    spreadsheet_token: &str,
    params: MoveDimensionParams,
) -> SDKResult<MoveDimensionResponse> {
    move_dimension_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 移动行列（带选项）
///
/// 根据 spreadsheetToken 将指定范围的行或列移动到新位置。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/moving-rows-or-columns
pub async fn move_dimension_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: MoveDimensionParams,
    option: RequestOption,
) -> SDKResult<MoveDimensionResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.source_range, "源范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::MoveDimension(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<MoveDimensionResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "移动行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "移动行列")
}

/// 替换范围
///
/// 根据 spreadsheetToken 和 range 替换指定范围的内容。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/replacing-a-range
pub async fn replace_range(
    config: &Config,
    spreadsheet_token: &str,
    params: ReplaceRangeParams,
) -> SDKResult<ReplaceRangeResponse> {
    replace_range_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 替换范围（带选项）
///
/// 根据 spreadsheetToken 和 range 替换指定范围的内容。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/replacing-a-range
pub async fn replace_range_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: ReplaceRangeParams,
    option: RequestOption,
) -> SDKResult<ReplaceRangeResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "替换范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ReplaceRange(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<ReplaceRangeResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "替换范围")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "替换范围")
}

/// 查找替换
///
/// 根据 spreadsheetToken 和 range 在指定范围内查找并替换内容。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/finding-and-replacing-values
pub async fn find_replace(
    config: &Config,
    spreadsheet_token: &str,
    params: FindReplaceParams,
) -> SDKResult<FindReplaceResponse> {
    find_replace_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 查找替换（带选项）
///
/// 根据 spreadsheetToken 和 range 在指定范围内查找并替换内容。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/finding-and-replacing-values
pub async fn find_replace_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: FindReplaceParams,
    option: RequestOption,
) -> SDKResult<FindReplaceResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "查找范围不能为空");
    validate_required!(params.find, "查找内容不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::FindReplace(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<FindReplaceResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "查找替换")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "查找替换")
}

/// 合并单元格
///
/// 根据 spreadsheetToken 和 range 合并指定范围的单元格。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/merging-cells
pub async fn merge_cells(
    config: &Config,
    spreadsheet_token: &str,
    params: MergeCellsParams,
) -> SDKResult<MergeCellsResponse> {
    merge_cells_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 合并单元格（带选项）
///
/// 根据 spreadsheetToken 和 range 合并指定范围的单元格。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/merging-cells
pub async fn merge_cells_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: MergeCellsParams,
    option: RequestOption,
) -> SDKResult<MergeCellsResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "合并范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::MergeCells(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<MergeCellsResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "合并单元格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "合并单元格")
}

/// 取消合并单元格
///
/// 根据 spreadsheetToken 和 range 取消合并指定范围的单元格。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/unmerging-cells
pub async fn unmerge_cells(
    config: &Config,
    spreadsheet_token: &str,
    params: UnmergeCellsParams,
) -> SDKResult<UnmergeCellsResponse> {
    unmerge_cells_with_options(config, spreadsheet_token, params, RequestOption::default()).await
}

/// 取消合并单元格（带选项）
///
/// 根据 spreadsheetToken 和 range 取消合并指定范围的单元格。
/// docPath: /document/server-docs/docs/sheets-v3/sheet-operations/unmerging-cells
pub async fn unmerge_cells_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UnmergeCellsParams,
    option: RequestOption,
) -> SDKResult<UnmergeCellsResponse> {
    // 验证必填字段
    validate_required!(spreadsheet_token.trim(), "表格Token不能为空");
    validate_required!(params.range, "取消合并范围不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UnmergeCells(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<UnmergeCellsResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "取消合并单元格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "取消合并单元格")
}

// API函数已经在模块中定义，不需要重复导出

// 模型已在同一个模块中定义，不需要重新导出

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../dimensionRange/delete → DeleteRangeResponse。
    #[tokio::test]
    async fn test_delete_range_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/token001/dimensionRange/delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = delete_range(
            &config,
            "token001",
            DeleteRangeParams {
                range: "Sheet1!A1:A5".into(),
                dimension: "ROWS".into(),
            },
        )
        .await
        .expect("删除范围应成功");
        assert!(resp.data.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/token001/dimensionRange/delete"
        );
    }
}
