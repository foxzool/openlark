/// 更新筛选
///
/// 更新子表筛选范围中的列筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update
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

/// 更新筛选请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterRequest {
    /// 列标识。
    pub col: String,
    /// 新的筛选条件。
    pub condition: Condition,
}

/// 更新筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFilterResponse {}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选
///
/// 更新指定子表的列筛选条件。
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterRequest,
) -> SDKResult<UpdateFilterResponse> {
    update_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选（带选项）
///
/// 更新指定子表的列筛选条件，并允许传入自定义请求选项。
pub async fn update_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterRequest,
    option: RequestOption,
) -> SDKResult<UpdateFilterResponse> {
    let api_endpoint =
        SheetsApiV3::UpdateFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<UpdateFilterResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新筛选")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT .../sheets/{sheet_id}/filter → UpdateFilterResponse（空 data）。
    #[tokio::test]
    async fn test_update_filter_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
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

        update_filter(
            &config,
            "tokenAbc",
            "sheetId001",
            UpdateFilterRequest {
                col: "0".into(),
                condition: Condition {
                    column_id: "A".into(),
                    operator: "contains".into(),
                    value: None,
                    ignore_case: None,
                },
            },
        )
        .await
        .expect("更新筛选应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter"
        );
        // 校验请求体透传
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["col"], "0");
        assert_eq!(sent["condition"]["operator"], "contains");
    }
}
