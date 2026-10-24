//! 查询指定时间范围公司版本
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/organization-management/company/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询指定时间范围公司版本请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineBody {
    /// 公司 ID 列表（必填，长度范围 1～10）
    pub company_ids: Vec<String>,
    /// 查询开始时间（包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 查询结束时间（不包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 返回数据的字段列表，可选 company_name 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 查询指定时间范围公司版本的响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineResponse {
    /// 列表项。
    pub items: Vec<serde_json::Value>,
}
impl ApiResponseTrait for QueryMultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 查询指定时间范围公司版本的请求。
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    config: Arc<Config>,
    body: QueryMultiTimelineBody,
}
/// 查询指定时间范围公司版本请求构建器实现。
impl QueryMultiTimelineRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryMultiTimelineBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: QueryMultiTimelineBody) -> Self {
        self.body = body;
        self
    }
    /// 设置公司 ID 列表（必填，长度范围 1～10）。
    pub fn company_ids(mut self, company_ids: Vec<String>) -> Self {
        self.body.company_ids = company_ids;
        self
    }
    /// 设置查询开始时间（包含），格式 "YYYY-MM-DD"。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.body.start_date = Some(start_date.into());
        self
    }
    /// 设置查询结束时间（不包含），格式 "YYYY-MM-DD"。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = Some(end_date.into());
        self
    }
    /// 设置返回数据的字段列表。
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.body.fields = Some(fields);
        self
    }
    /// 执行查询指定时间范围公司版本请求。
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        validate_required!(self.body.company_ids, "company_ids 不能为空且为 1～10 个");
        if self.body.company_ids.len() > 10 {
            return Err(CoreError::validation_msg("company_ids 长度范围为 1～10"));
        }

        let request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/companies/query_multi_timeline",
        )
        .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询指定时间范围公司版本",
        )
        .await
    }
}
