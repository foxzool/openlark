/// 获取筛选视图
///
/// 获取指定筛选视图 id 的名字和筛选范围。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::api_endpoints::SheetsApiV3;

/// 获取筛选视图响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewResponse {
    /// 筛选视图详情。
    pub filter_view: FilterView,
}

impl ApiResponseTrait for GetFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选视图。
pub async fn get_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<GetFilterViewResponse> {
    get_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选视图（支持请求选项）。
pub async fn get_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterViewResponse> {
    let api_endpoint = SheetsApiV3::GetFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterViewResponse> = api_endpoint.to_request();

    Transport::request_typed(api_request, config, Some(option), "获取筛选视图").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../filter_views/{filter_view_id} → GetFilterViewResponse（filter_view）。
    #[tokio::test]
    async fn test_get_filter_view_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "filter_view": {
                        "filter_view_id": "fv001",
                        "name": "筛选视图1",
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

        let resp = get_filter_view(&config, "tokenAbc", "sheetId001", "fv001")
            .await
            .expect("获取筛选视图应成功");
        assert_eq!(resp.filter_view.filter_view_id, "fv001");
        assert_eq!(resp.filter_view.name, "筛选视图1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001"
        );
    }
}
