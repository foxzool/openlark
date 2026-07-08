/// 获取筛选条件
///
/// 获取筛选视图某列的筛选条件信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::Condition;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选条件响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterConditionResponse {
    /// 筛选条件详情。
    pub condition: Condition,
}

impl ApiResponseTrait for GetFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选条件。
pub async fn get_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
) -> SDKResult<GetFilterConditionResponse> {
    get_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        condition_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选条件（带请求选项）。
pub async fn get_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::GetFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterConditionResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选条件")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../conditions/{condition_id} → GetFilterConditionResponse（condition）。
    #[tokio::test]
    async fn test_get_filter_condition_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "condition": { "filter_view_id": "fv001", "column_id": "E", "operator": "equals" }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = get_filter_condition(&config, "tokenAbc", "sheetId001", "fv001", "E")
            .await
            .expect("获取筛选条件应成功");
        assert_eq!(resp.condition.column_id, "E");
        assert_eq!(resp.condition.operator, "equals");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"
        );
    }
}
