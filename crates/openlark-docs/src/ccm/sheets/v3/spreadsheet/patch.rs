/// 更新电子表格
///
/// 更新电子表格的基本信息，如标题、时区、语言等。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/patch
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for UpdateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新电子表格
///
/// 更新电子表格的基本信息，如标题、时区、语言等。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/patch
pub async fn update_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSpreadsheetParams,
) -> SDKResult<UpdateSpreadsheetResponse> {
    update_spreadsheet_with_options(config, spreadsheet_token, params, RequestOption::default())
        .await
}

/// 更新电子表格（支持自定义选项）
pub async fn update_spreadsheet_with_options(
    config: &Config,
    spreadsheet_token: &str,
    params: UpdateSpreadsheetParams,
    option: RequestOption,
) -> SDKResult<UpdateSpreadsheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = SheetsApiV3::PatchSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<UpdateSpreadsheetResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新电子表格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新电子表格")
}
