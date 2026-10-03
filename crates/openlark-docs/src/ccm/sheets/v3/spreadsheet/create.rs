/// 创建电子表格
///
/// 创建并初始化一个新的电子表格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建电子表格
///
/// 创建并初始化一个新的电子表格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
pub async fn create_spreadsheet(
    config: &Config,
    params: CreateSpreadsheetParams,
) -> SDKResult<CreateSpreadsheetResponse> {
    create_spreadsheet_with_options(config, params, RequestOption::default()).await
}

/// 创建电子表格（支持自定义选项）
pub async fn create_spreadsheet_with_options(
    config: &Config,
    params: CreateSpreadsheetParams,
    option: RequestOption,
) -> SDKResult<CreateSpreadsheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = SheetsApiV3::CreateSpreadsheet;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateSpreadsheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "创建电子表格")?);

    // 发送请求并提取响应数据
    Transport::request_typed(api_request, config, Some(option), "创建电子表格").await
}
