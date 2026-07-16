/// 移动行列
///
/// 移动工作表中的行或列。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for MoveDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动行列
///
/// 移动工作表中的行或列。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension
pub async fn move_dimension(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: MoveDimensionParams,
) -> SDKResult<MoveDimensionResponse> {
    move_dimension_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 移动行列（带请求选项）
///
/// 移动工作表中的行或列。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension
pub async fn move_dimension_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: MoveDimensionParams,
    option: RequestOption,
) -> SDKResult<MoveDimensionResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint =
        SheetsApiV3::MoveDimension(spreadsheet_token.to_string(), sheet_id.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<MoveDimensionResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "移动行列")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "移动行列")
}
