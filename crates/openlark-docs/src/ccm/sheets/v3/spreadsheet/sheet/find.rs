/// 查找单元格
///
/// 在指定范围内查找符合条件的单元格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use super::super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for FindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找单元格
pub async fn find_cells(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: FindParams,
) -> SDKResult<FindResponse> {
    find_cells_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 查找单元格（带选项）
pub async fn find_cells_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: FindParams,
    option: RequestOption,
) -> SDKResult<FindResponse> {
    let api_endpoint = SheetsApiV3::FindCells(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<FindResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "查找单元格")?);

    Transport::request_typed(api_request, config, Some(option), "查找单元格").await
}
