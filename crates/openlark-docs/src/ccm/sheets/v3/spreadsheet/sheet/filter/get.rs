/// 获取筛选
///
/// 获取子表的详细筛选信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::SheetFilterInfo;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterResponse {
    /// 子表筛选详情。
    pub sheet_filter_info: SheetFilterInfo,
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选
///
/// 获取指定子表的筛选配置。
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<GetFilterResponse> {
    get_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选（带选项）
///
/// 获取指定子表的筛选配置，并允许传入自定义请求选项。
pub async fn get_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterResponse> {
    let api_endpoint = SheetsApiV3::GetFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<GetFilterResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../sheets/{sheet_id}/filter → GetFilterResponse（sheet_filter_info）。
    #[tokio::test]
    async fn test_get_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "sheet_filter_info": {
                        "filter_id": "filter_001",
                        "range": "A1:A10"
                    }
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

        let resp = get_filter(&config, "tokenAbc", "sheetId001")
            .await
            .expect("获取筛选应成功");
        assert_eq!(resp.sheet_filter_info.filter_id, "filter_001");
        assert_eq!(resp.sheet_filter_info.range, "A1:A10");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter"
        );
    }
}
