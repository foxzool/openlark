//! 批量获取员工列表
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/filter>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/filter>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工列表 Builder
#[derive(Debug, Clone)]
pub struct EmployeeFilterRequestBuilder {
    config: Config,
    /// 查询条件
    filter: MultiFilterCondition,
    /// 需要查询的字段列表
    required_fields: Vec<String>,
    /// 分页信息
    page_request: PageCondition,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id/employee_id）
    employee_id_type: Option<String>,
    /// 部门 ID 类型（查询参数，可选：department_id/open_department_id）
    department_id_type: Option<String>,
}

impl EmployeeFilterRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            filter: MultiFilterCondition::default(),
            required_fields: Vec::new(),
            page_request: PageCondition::default(),
            employee_id_type: None,
            department_id_type: None,
        }
    }

    /// 添加比较表达式（多个表达式之间的关系为「且」）。
    ///
    /// `field` 支持 `base_info.mobile`、`base_info.email`、
    /// `base_info.departments.department_id` + `work_info.staff_status`（需同时指定）、
    /// `work_info.job_number`；`operator` 可选 `eq`/`in`；
    /// `value` 为转义后的 JSON 字符串（如 `"\"+8613000000001\""`）。
    pub fn condition(
        mut self,
        field: impl Into<String>,
        operator: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.filter.conditions.push(FilterCondition {
            field: field.into(),
            operator: operator.into(),
            value: value.into(),
        });
        self
    }

    /// 设置比较表达式列表
    pub fn conditions(mut self, conditions: Vec<FilterCondition>) -> Self {
        self.filter.conditions = conditions;
        self
    }

    /// 添加需要查询的字段（如 "base_info.gender"）
    pub fn required_field(mut self, field: impl Into<String>) -> Self {
        self.required_fields.push(field.into());
        self
    }

    /// 设置需要查询的字段列表
    pub fn required_fields(mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.required_fields = fields.into_iter().map(Into::into).collect();
        self
    }

    /// 设置本次请求条数（最小 0 条，最大 100 条，默认 20）
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（第一次传空或不传，后续传上一次返回的 page_token）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_request.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）
    pub fn employee_id_type(mut self, employee_id_type: impl Into<String>) -> Self {
        self.employee_id_type = Some(employee_id_type.into());
        self
    }

    /// 设置部门 ID 类型（查询参数）
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeFilterResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeFilterResponse> {
        validate_required!(self.filter.conditions, "filter.conditions 不能为空");
        validate_required!(self.required_fields, "required_fields 不能为空");

        let url = "/open-apis/directory/v1/employees/filter";

        let body = EmployeeFilterBody {
            filter: self.filter,
            required_fields: self.required_fields,
            page_request: self.page_request,
        };

        let req: ApiRequest<EmployeeFilterResponse> = ApiRequest::post(url)
            .body(serde_json::to_value(&body)?)
            .query_opt("employee_id_type", self.employee_id_type)
            .query_opt("department_id_type", self.department_id_type);
        Transport::request_typed(req, &self.config, Some(option), "批量获取员工列表").await
    }
}

/// 批量获取员工列表请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeFilterBody {
    /// 查询条件
    pub filter: MultiFilterCondition,
    /// 需要查询的字段列表，如 ["base_info.gender"]（不传则不会返回任何字段）
    pub required_fields: Vec<String>,
    /// 分页信息
    pub page_request: PageCondition,
}

/// 查询条件（官方 multi_filter_condition）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiFilterCondition {
    /// 比较表达式列表（0~10 个），多个表达式之间的关系为「且」
    pub conditions: Vec<FilterCondition>,
}

/// 比较表达式（官方 filter_condition）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterCondition {
    /// 筛选条件的左值，如 `base_info.mobile`
    pub field: String,
    /// 比较操作符，可选 `eq`（等于）/ `in`（属于任一）
    pub operator: String,
    /// 筛选条件的右值，为转义后的 JSON 字符串（如 `"\"+8613000000001\""`）
    pub value: String,
}

/// 分页参数（官方 page_condition）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageCondition {
    /// 本次请求条数，最小 0 条，最大 100 条，默认 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 分页标记，顺序分页查询，第一次传空或不传
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 员工简要信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeBrief {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 批量获取员工列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeFilterResponse {
    /// 员工列表
    #[serde(rename = "items")]
    pub items: Vec<EmployeeBrief>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for EmployeeFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeFilterRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeFilterBuilder = EmployeeFilterRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST 官方 filter + required_fields + page_request 请求体 → 强类型 EmployeeFilterResponse。
    #[tokio::test]
    async fn test_filter_employees_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/filter"))
            .and(body_json(json!({
                "filter": {
                    "conditions": [
                        {
                            "field": "base_info.mobile",
                            "operator": "eq",
                            "value": "\"8619922333322\""
                        }
                    ]
                },
                "required_fields": ["base_info.gender"],
                "page_request": {
                    "page_size": 20
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employee_id": "emp_001",
                            "name": "alice",
                            "department_ids": ["dept_001"]
                        }
                    ],
                    "has_more": true,
                    "page": 1,
                    "page_size": 20
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

        let resp = EmployeeFilterRequestBuilder::new(config)
            .condition("base_info.mobile", "eq", "\"8619922333322\"")
            .required_field("base_info.gender")
            .page_size(20)
            .execute()
            .await
            .expect("批量获取员工列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].employee_id, "emp_001");
        assert_eq!(resp.items[0].name, "alice");
        assert!(resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/filter"
        );
        assert_eq!(received[0].method, "POST");
    }

    /// 必填校验：conditions 为空时不发请求。
    #[tokio::test]
    async fn test_filter_employees_validates_required_fields() {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = EmployeeFilterRequestBuilder::new(config)
            .required_field("base_info.gender")
            .execute()
            .await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
