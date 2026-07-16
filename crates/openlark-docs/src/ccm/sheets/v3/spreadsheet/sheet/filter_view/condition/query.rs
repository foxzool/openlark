/// 查询筛选条件
///
/// 查询一个筛选视图的所有筛选条件，返回筛选视图的筛选范围内的筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query
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

/// 查询筛选条件响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterConditionsResponse {
    /// 筛选条件列表。
    pub items: Vec<Condition>,
}

impl ApiResponseTrait for QueryFilterConditionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选条件。
pub async fn query_filter_conditions(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<QueryFilterConditionsResponse> {
    query_filter_conditions_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        RequestOption::default(),
    )
    .await
}

/// 查询筛选条件（支持请求选项）。
pub async fn query_filter_conditions_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    option: RequestOption,
) -> SDKResult<QueryFilterConditionsResponse> {
    let api_endpoint = SheetsApiV3::QueryFilterConditions(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<QueryFilterConditionsResponse> = api_endpoint.to_request();

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "查询筛选条件")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../conditions/query → QueryFilterConditionsResponse（items）。
    #[tokio::test]
    async fn test_query_filter_conditions_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "filter_view_id": "fv001", "column_id": "E", "operator": "equals" },
                        { "filter_view_id": "fv001", "column_id": "F", "operator": "contains" }
                    ]
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

        let resp = query_filter_conditions(&config, "tokenAbc", "sheetId001", "fv001")
            .await
            .expect("查询筛选条件应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].column_id, "E");
        assert_eq!(resp.items[1].operator, "contains");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/query"
        );
    }
}
