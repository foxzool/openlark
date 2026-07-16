/// 更新筛选条件
///
/// 更新筛选视图范围的某列的筛选条件，condition_id 即为列的字母号。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update
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

/// 更新筛选条件请求体（字段在文档中标注为非必填，可按需传入）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFilterConditionRequest {
    /// 筛选类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    /// 比较类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    /// 期望值列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

/// 更新筛选条件响应体 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterConditionResponse {
    /// 更新后的筛选条件。
    pub condition: Condition,
}

impl ApiResponseTrait for UpdateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选条件。
pub async fn update_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    params: UpdateFilterConditionRequest,
) -> SDKResult<UpdateFilterConditionResponse> {
    update_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        condition_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选条件（带请求选项）。
pub async fn update_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    params: UpdateFilterConditionRequest,
    option: RequestOption,
) -> SDKResult<UpdateFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::UpdateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFilterConditionResponse> = api_endpoint
        .to_request()
        .body(serialize_params(&params, "更新筛选条件")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选条件")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT .../conditions/{condition_id} → UpdateFilterConditionResponse（condition）。
    #[tokio::test]
    async fn test_update_filter_condition_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "condition": { "filter_view_id": "fv001", "column_id": "E", "operator": "contains" }
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

        let resp = update_filter_condition(
            &config,
            "tokenAbc",
            "sheetId001",
            "fv001",
            "E",
            UpdateFilterConditionRequest {
                filter_type: Some("noCheck".into()),
                compare_type: None,
                expected: Some(vec!["v1".into()]),
            },
        )
        .await
        .expect("更新筛选条件应成功");
        assert_eq!(resp.condition.operator, "contains");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/sheets/v3/spreadsheets/tokenAbc/sheets/sheetId001/filter_views/fv001/conditions/E"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["filter_type"], "noCheck");
    }
}
