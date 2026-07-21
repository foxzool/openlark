/// 删除筛选
///
/// 删除子表的筛选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::SheetsApiV3;

/// 删除筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFilterResponse {}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<DeleteFilterResponse> {
    delete_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        RequestOption::default(),
    )
    .await
}

/// 删除筛选（带选项）
pub async fn delete_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    option: RequestOption,
) -> SDKResult<DeleteFilterResponse> {
    let api_endpoint =
        SheetsApiV3::DeleteFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<DeleteFilterResponse> = api_endpoint.to_request();

    Transport::request_typed(api_request, config, Some(option), "删除筛选").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../sheets/{sheet_id}/filter → DeleteFilterResponse（空 data）。
    #[tokio::test]
    async fn test_delete_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter",
            ))
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

        delete_filter(&config, "tokenAbc", "sheetId001")
            .await
            .expect("删除筛选应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter"
        );
    }
}
