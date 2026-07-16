/// 获取电子表格信息
///
/// 获取指定电子表格的详细信息，包括基本属性和配置。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取电子表格信息
///
/// 获取指定电子表格的详细信息，包括基本属性和配置。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get
pub async fn get_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<GetSpreadsheetResponse> {
    get_spreadsheet_with_options(config, spreadsheet_token, RequestOption::default()).await
}

/// 获取电子表格信息（支持自定义选项）
pub async fn get_spreadsheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    option: RequestOption,
) -> SDKResult<GetSpreadsheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = SheetsApiV3::GetSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetSpreadsheetResponse> = api_endpoint.to_request();

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取电子表格信息")
}
