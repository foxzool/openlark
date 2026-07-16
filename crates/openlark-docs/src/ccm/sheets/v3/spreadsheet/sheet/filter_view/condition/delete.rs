/// 删除筛选条件
///
/// 删除筛选视图的筛选范围某一列的筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除筛选条件响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFilterConditionResponse {}

impl ApiResponseTrait for DeleteFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选条件
pub async fn delete_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
) -> SDKResult<DeleteFilterConditionResponse> {
    delete_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        condition_id,
        RequestOption::default(),
    )
    .await
}

/// 删除筛选条件（支持请求选项）
pub async fn delete_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    option: RequestOption,
) -> SDKResult<DeleteFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::DeleteFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<DeleteFilterConditionResponse> = api_endpoint.to_request();

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除筛选条件")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../conditions/{condition_id} → DeleteFilterConditionResponse（空 data）。
    #[tokio::test]
    async fn test_delete_filter_condition_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        delete_filter_condition(&config, "tokenAbc", "sheetId001", "fv001", "E")
            .await
            .expect("删除筛选条件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"
        );
    }
}
