/// 更新筛选视图
///
/// 更新筛选视图的名字或者筛选范围。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/patch
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

/// 更新筛选视图请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// 筛选视图名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    /// 筛选范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

/// 更新筛选视图响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewResponse {
    /// 更新后的筛选视图。
    pub filter_view: FilterView,
}

impl ApiResponseTrait for UpdateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选视图。
pub async fn update_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: UpdateFilterViewRequest,
) -> SDKResult<UpdateFilterViewResponse> {
    update_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选视图（支持自定义选项）。
pub async fn update_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: UpdateFilterViewRequest,
    option: RequestOption,
) -> SDKResult<UpdateFilterViewResponse> {
    let api_endpoint = SheetsApiV3::PatchFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFilterViewResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新筛选视图")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选视图")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../filter_views/{filter_view_id} → UpdateFilterViewResponse（filter_view）。
    #[tokio::test]
    async fn test_update_filter_view_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "filter_view": {
                        "filter_view_id": "fv001",
                        "name": "新名称",
                        "range": "B1:B10"
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

        let resp = update_filter_view(
            &config,
            "tokenAbc",
            "sheetId001",
            "fv001",
            UpdateFilterViewRequest {
                filter_view_name: Some("新名称".into()),
                range: Some("B1:B10".into()),
            },
        )
        .await
        .expect("更新筛选视图应成功");
        assert_eq!(resp.filter_view.name, "新名称");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["filter_view_name"], "新名称");
    }
}
