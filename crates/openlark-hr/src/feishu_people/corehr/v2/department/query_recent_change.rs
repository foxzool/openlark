//! 查询生效信息变更部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_recent_change>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// QueryRecentChangeRequest
#[derive(Debug, Clone)]
pub struct QueryRecentChangeRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（必填，官方类型 int，最大支持单次 2000 条变更，经 query string 以字符串发送）
    page_size: String,
    /// 查询的开始时间（必填，格式：YYYY-MM-DD HH:MM:SS，UTC+8 时区）
    start_date: String,
    /// 查询的结束时间（必填，格式：YYYY-MM-DD HH:MM:SS，须大于开始时间）
    end_date: String,
    /// 分页标记，第一次请求不填
    page_token: Option<String>,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
}

impl QueryRecentChangeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            page_token: None,
            department_id_type: None,
        }
    }

    /// 设置分页大小（必填，取值范围 1 ～ 2000）。
    pub fn page_size(mut self, page_size: impl Into<String>) -> Self {
        self.page_size = page_size.into();
        self
    }

    /// 设置查询的开始时间（必填，格式：YYYY-MM-DD HH:MM:SS）。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    /// 设置查询的结束时间（必填，格式：YYYY-MM-DD HH:MM:SS）。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryRecentChangeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryRecentChangeResponse> {
        validate_required!(self.page_size, "page_size 不能为空");
        validate_required!(self.start_date, "start_date 不能为空");
        validate_required!(self.end_date, "end_date 不能为空");

        let request = ApiRequest::<QueryRecentChangeResponse>::get(
            "/open-apis/corehr/v2/departments/query_recent_change",
        )
        .query("page_size", self.page_size)
        .query("start_date", self.start_date)
        .query("end_date", self.end_date)
        .query_opt("page_token", self.page_token)
        .query_opt("department_id_type", self.department_id_type);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// QueryRecentChangeResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRecentChangeResponse {
    /// 部门 ID 列表，其中包含新建、更新以及删除的 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 目标查询时间范围内被删除的部门列表（department_ids 的子集）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_department_ids: Option<Vec<String>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for QueryRecentChangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/departments/query_recent_change
    /// （官方查询参数 page_size/start_date/end_date 必填；
    ///   data 内层键为 department_ids / deleted_department_ids / page_token / has_more）
    #[tokio::test]
    async fn test_query_recent_change_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/departments/query_recent_change"))
            .and(query_param("page_size", "100"))
            .and(query_param("start_date", "2024-06-01 00:00:00"))
            .and(query_param("end_date", "2024-08-01 02:10:10"))
            .and(query_param(
                "department_id_type",
                "people_corehr_department_id",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_ids": [
                        "7094136522860922111"
                    ],
                    "deleted_department_ids": [
                        "7094136522860922111"
                    ],
                    "page_token": "7094136522860922111",
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

        let resp = QueryRecentChangeRequest::new(config)
            .page_size("100")
            .start_date("2024-06-01 00:00:00")
            .end_date("2024-08-01 02:10:10")
            .department_id_type("people_corehr_department_id")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(
            resp.department_ids,
            Some(vec!["7094136522860922111".to_string()])
        );
        assert_eq!(
            resp.deleted_department_ids,
            Some(vec!["7094136522860922111".to_string()])
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
