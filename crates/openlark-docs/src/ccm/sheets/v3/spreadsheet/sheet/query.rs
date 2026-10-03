/// 查询工作表
///
/// 查询指定电子表格中的所有工作表信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::super::models::*;
use crate::common::api_endpoints::SheetsApiV3;

impl ApiResponseTrait for QuerySheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询工作表
///
/// 查询指定电子表格中的所有工作表信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query
pub async fn query_sheets(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<QuerySheetResponse> {
    query_sheets_with_options(config, spreadsheet_token, RequestOption::default()).await
}

/// 查询工作表（支持自定义选项）
pub async fn query_sheets_with_options(
    config: &Config,
    spreadsheet_token: &str,
    option: RequestOption,
) -> SDKResult<QuerySheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = SheetsApiV3::QuerySheets(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<QuerySheetResponse> = api_endpoint.to_request();

    // 发送请求并提取响应数据
    Transport::request_typed(api_request, config, Some(option), "查询工作表").await
}
