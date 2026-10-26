//! 批量查询部门操作日志
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_operation_logs>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required, validate_required_list,
};

use serde::{Deserialize, Serialize};

/// 批量查询部门操作日志请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryOperationLogsBody {
    /// 部门 ID 列表（必填，1～100 个）
    pub department_ids: Vec<String>,
    /// 查询的起始操作日期（必填，格式 "YYYY-MM-DD"，不含时分秒）
    pub start_date: String,
    /// 查询的截止操作日期（必填，格式 "YYYY-MM-DD"，起止跨度最大 366 天）
    pub end_date: String,
}

impl QueryOperationLogsBody {
    /// 创建请求体
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置部门 ID 列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = department_ids;
        self
    }

    /// 设置查询的起始操作日期（格式 "YYYY-MM-DD"）
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = start_date;
        self
    }

    /// 设置查询的截止操作日期（格式 "YYYY-MM-DD"）
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = end_date;
        self
    }
}

/// 批量查询部门操作日志响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryOperationLogsResponse {
    /// 操作日志列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op_logs: Option<Vec<OpLog>>,
    /// 下一页 token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 操作日志（官方类型 organization_op_log）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpLog {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 字段变化列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<OperationLogEntityField>>,
    /// 操作人工号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 操作类型：10 新建部门记录 / 20 新建、复制部门版本 / 50 编辑部门版本 / 60 撤销部门版本 / 70 停用部门 / 80 启用部门 / 90 删除部门记录 / 100 自定义部门排序 / 110 更新部门 / 120 编辑部门关联规则
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<i32>,
    /// 操作时间（带时分秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_time: Option<String>,
    /// 操作生效时间（被操作部门版本的生效日期）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 操作原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_reason: Option<String>,
    /// 调整原因列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_reasons: Option<Vec<String>>,
}

/// 字段变化（官方类型 operation_log_entity_field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationLogEntityField {
    /// 变更字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 旧值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 新值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ApiResponseTrait for QueryOperationLogsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门操作日志请求构建器
#[derive(Debug, Clone)]
pub struct QueryOperationLogsRequestBuilder {
    config: Config,
    body: QueryOperationLogsBody,
    /// 分页大小（必填，最大 1000，取值范围 1～1000）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 部门 ID 类型（open_department_id/department_id/people_corehr_department_id）
    department_id_type: Option<String>,
}

impl QueryOperationLogsRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: QueryOperationLogsBody::new(),
            page_size: None,
            page_token: None,
            department_id_type: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: QueryOperationLogsBody) -> Self {
        self.body = body;
        self
    }

    /// 设置部门 ID 列表（必填，1～100 个）
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.body.department_ids = department_ids;
        self
    }

    /// 设置查询的起始操作日期（必填，格式 "YYYY-MM-DD"）
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.body.start_date = start_date.into();
        self
    }

    /// 设置查询的截止操作日期（必填，格式 "YYYY-MM-DD"）
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = end_date.into();
        self
    }

    /// 设置分页大小（必填，取值范围 1～1000）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryOperationLogsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryOperationLogsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=1000).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～1000"));
        }
        validate_required_list!(
            self.body.department_ids,
            100,
            "department_ids 不能为空且最多 100 个"
        );
        validate_required!(self.body.start_date, "start_date 不能为空");
        validate_required!(self.body.end_date, "end_date 不能为空");

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentQueryOperationLogs;
        let request = ApiRequest::<QueryOperationLogsResponse>::post(api_endpoint.to_url())
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body)?);

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询部门操作日志响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/departments/query_operation_logs
    #[tokio::test]
    async fn test_query_operation_logs_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/departments/query_operation_logs",
            ))
            .and(query_param("page_size", "100"))
            .and(body_json(json!({
                "department_ids": ["7094136522860922222"],
                "start_date": "2023-01-01",
                "end_date": "2024-01-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "op_logs": [
                        {
                            "object_id": "7094136522860922111",
                            "changes": [
                                {
                                    "field": "description",
                                    "before": "优秀部门",
                                    "after": "更优秀的部门"
                                }
                            ],
                            "operator": "E001",
                            "operation_type": 10,
                            "operation_time": "2023-11-15 19:25:45",
                            "effective_time": "2023-10-28",
                            "operation_reason": "因人员调整,变更部门负责人为张三",
                            "change_reasons": ["新建部门"]
                        }
                    ],
                    "next_page_token": "2",
                    "has_more": true
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

        let resp = QueryOperationLogsRequestBuilder::new(config)
            .department_ids(vec!["7094136522860922222".to_string()])
            .start_date("2023-01-01")
            .end_date("2024-01-01")
            .page_size(100)
            .execute()
            .await
            .expect("请求应成功");

        let logs = resp.op_logs.expect("应解析操作日志列表");
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].object_id, Some("7094136522860922111".to_string()));
        assert_eq!(logs[0].operator, Some("E001".to_string()));
        assert_eq!(logs[0].operation_type, Some(10));
        let changes = logs[0].changes.as_deref().expect("应解析字段变化列表");
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].field, Some("description".to_string()));
        assert_eq!(changes[0].before, Some("优秀部门".to_string()));
        assert_eq!(changes[0].after, Some("更优秀的部门".to_string()));
        assert_eq!(logs[0].change_reasons, Some(vec!["新建部门".to_string()]));
        assert_eq!(resp.next_page_token, Some("2".to_string()));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
