/// 查询筛选视图
///
/// 查询子表内所有的筛选视图基本信息，包括 id、name 和 range。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 查询筛选视图响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsResponse {
    /// 筛选视图列表。
    pub items: Vec<FilterView>,
}

impl ApiResponseTrait for QueryFilterViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选视图。
pub async fn query_filter_views(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFilterViewsResponse> {
    query_filter_views_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        RequestOption::default(),
    )
    .await
}

/// 查询筛选视图（支持自定义选项）。
pub async fn query_filter_views_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    option: RequestOption,
) -> SDKResult<QueryFilterViewsResponse> {
    let api_endpoint =
        SheetsApiV3::QueryFilterViews(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<QueryFilterViewsResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "查询筛选视图")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../sheets/{sheet_id}/filter_views/query → QueryFilterViewsResponse（items）。
    #[tokio::test]
    async fn test_query_filter_views_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "filter_view_id": "fv001", "name": "视图1", "range": "A1:A10" },
                        { "filter_view_id": "fv002", "name": "视图2", "range": "B1:B10" }
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

        let resp = query_filter_views(&config, "tokenAbc", "sheetId001")
            .await
            .expect("查询筛选视图应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].filter_view_id, "fv001");
        assert_eq!(resp.items[1].range, "B1:B10");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/query"
        );
    }
}
