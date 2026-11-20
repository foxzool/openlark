//! 搜索被动解除补偿金信息。
//!
//! `POST /open-apis/corehr/v2/severance_pays/query`，只接受 tenant access token。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/severance_pay/query>

use super::models::{SeverancePay, optional_list, optional_text};
use crate::common::api_endpoints::FeishuPeopleApiV2;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use openlark_core::{CoreError, SDKResult};
use serde::{Deserialize, Serialize};

/// 搜索被动解除补偿金信息。
#[derive(Debug, Clone)]
pub struct QueryRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    body: QueryRequestBody,
}

/// 搜索被动解除补偿金信息的请求体。筛选项都可选。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QueryRequestBody {
    /// 员工雇佣 ID 列表。最多 100 个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_ids: Option<Vec<String>>,
    /// 补偿金业务单 ID 列表。最多 100 个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severance_pay_ids: Option<Vec<String>>,
    /// 补偿金状态。`approving`、`approved`、`rejected`、`withdrawn`。最多 100 个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// 测算状态。最多 100 个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_statuses: Option<Vec<String>>,
    /// 申请发起时间起，Unix 秒级时间戳。长度 1 到 19。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time_start: Option<String>,
    /// 申请发起时间止，Unix 秒级时间戳。长度 1 到 19。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time_end: Option<String>,
    /// 离职日期起。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_date_start: Option<String>,
    /// 离职日期止。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_date_end: Option<String>,
    /// 指定返回字段。最多 1000 个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<Vec<String>>,
}

/// 搜索被动解除补偿金信息的响应数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 补偿金记录。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SeverancePay>>,
    /// 下一页标记。没有更多项时不返回。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 符合条件的总数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
            body: QueryRequestBody::default(),
        }
    }

    /// 分页大小。取值 1 到 100。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记。第一次请求不传。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 部门 ID 类型。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 员工雇佣 ID 列表。
    pub fn employment_ids(mut self, employment_ids: Vec<String>) -> Self {
        self.body.employment_ids = Some(employment_ids);
        self
    }

    /// 补偿金业务单 ID 列表。
    pub fn severance_pay_ids(mut self, severance_pay_ids: Vec<String>) -> Self {
        self.body.severance_pay_ids = Some(severance_pay_ids);
        self
    }

    /// 补偿金状态过滤。
    pub fn statuses(mut self, statuses: Vec<String>) -> Self {
        self.body.statuses = Some(statuses);
        self
    }

    /// 测算状态过滤。
    pub fn calculate_statuses(mut self, calculate_statuses: Vec<String>) -> Self {
        self.body.calculate_statuses = Some(calculate_statuses);
        self
    }

    /// 申请发起时间起。
    pub fn apply_initiating_time_start(mut self, value: impl Into<String>) -> Self {
        self.body.apply_initiating_time_start = Some(value.into());
        self
    }

    /// 申请发起时间止。
    pub fn apply_initiating_time_end(mut self, value: impl Into<String>) -> Self {
        self.body.apply_initiating_time_end = Some(value.into());
        self
    }

    /// 离职日期起。
    pub fn offboarding_date_start(mut self, value: impl Into<String>) -> Self {
        self.body.offboarding_date_start = Some(value.into());
        self
    }

    /// 离职日期止。
    pub fn offboarding_date_end(mut self, value: impl Into<String>) -> Self {
        self.body.offboarding_date_end = Some(value.into());
        self
    }

    /// 指定返回字段。
    pub fn select_fields(mut self, select_fields: Vec<String>) -> Self {
        self.body.select_fields = Some(select_fields);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<QueryResponse> {
        let Some(page_size) = self.page_size else {
            return Err(CoreError::validation_msg(
                "page_size 必须提供，取值 1 到 100",
            ));
        };
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg(
                "page_size 必须提供，取值 1 到 100",
            ));
        }
        optional_list(
            &self.body.employment_ids,
            100,
            "employment_ids 不能超过 100 个",
        )?;
        optional_list(
            &self.body.severance_pay_ids,
            100,
            "severance_pay_ids 不能超过 100 个",
        )?;
        optional_list(&self.body.statuses, 100, "statuses 不能超过 100 个")?;
        optional_list(
            &self.body.calculate_statuses,
            100,
            "calculate_statuses 不能超过 100 个",
        )?;
        optional_text(
            &self.body.apply_initiating_time_start,
            19,
            "apply_initiating_time_start 不能为空且不能超过 19 个字符",
        )?;
        optional_text(
            &self.body.apply_initiating_time_end,
            19,
            "apply_initiating_time_end 不能为空且不能超过 19 个字符",
        )?;
        optional_list(
            &self.body.select_fields,
            1000,
            "select_fields 不能超过 1000 个",
        )?;
        let request =
            ApiRequest::<QueryResponse>::post(FeishuPeopleApiV2::SeverancePayQuery.to_url())
                .with_supported_access_token_types(vec![AccessTokenType::Tenant])
                .query("page_size", page_size.to_string())
                .query_opt("page_token", self.page_token)
                .query_opt("user_id_type", self.user_id_type)
                .query_opt("department_id_type", self.department_id_type)
                .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索被动解除补偿金信息",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn config(server: &MockServer) -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build()
    }

    #[test]
    fn query_endpoint_is_severance_pay_query() {
        assert_eq!(
            FeishuPeopleApiV2::SeverancePayQuery.to_url(),
            "/open-apis/corehr/v2/severance_pays/query"
        );
    }

    #[tokio::test]
    async fn query_requires_page_size_before_http() {
        let server = MockServer::start().await;
        let err = QueryRequest::new(config(&server))
            .execute()
            .await
            .expect_err("缺少 page_size");
        assert!(err.to_string().contains("page_size"), "{err}");
        let received = server.received_requests().await.unwrap_or_default();
        assert!(received.is_empty());
    }

    #[tokio::test]
    async fn query_posts_filters_and_reads_item() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/severance_pays/query"))
            .and(query_param("page_size", "20"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .and(body_json(json!({
                "employment_ids": ["7659617611689510444"],
                "statuses": ["approved"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "severance_pay_id": "7660423757252773420",
                        "employment_id": "7659617611689510444",
                        "status": { "enum_name": "approved" },
                        "severance_pay_info": {
                            "currency": "THB",
                            "standard_compensation_unit": { "enum_name": "month" }
                        },
                        "custom_fields": [{
                            "custom_api_name": "name",
                            "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                            "type": 1,
                            "value": "\"231\""
                        }],
                        "reason_chain": ["voluntary"]
                    }],
                    "page_token": "page_token_xxx",
                    "has_more": false,
                    "total": "1"
                }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let response = QueryRequest::new(config(&server))
            .page_size(20)
            .employment_ids(vec!["7659617611689510444".to_string()])
            .statuses(vec!["approved".to_string()])
            .execute_with_options(
                RequestOption::builder()
                    .tenant_access_token("test-tenant-token")
                    .build(),
            )
            .await
            .expect("搜索应成功");
        assert_eq!(response.total.as_deref(), Some("1"));
        assert_eq!(response.has_more, Some(false));
        let item = response.items.expect("items").remove(0);
        assert_eq!(
            item.severance_pay_id.as_deref(),
            Some("7660423757252773420")
        );
        let info = item.severance_pay_info.expect("info");
        assert_eq!(info.pay.currency.as_deref(), Some("THB"));
        assert_eq!(
            info.standard_compensation_unit
                .and_then(|unit| unit.enum_name),
            Some("month".to_string())
        );
        let custom = item.custom_fields.expect("custom").remove(0);
        assert_eq!(custom.field_type, Some(1));
        assert_eq!(item.reason_chain.unwrap(), vec!["voluntary".to_string()]);
    }
}
