//! 查询指定时间范围地点版本
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/organization-management/location/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询指定时间范围地点版本请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineBody {
    /// 开始日期。
    pub from_date: Option<String>,
    /// 结束日期。
    pub to_date: Option<String>,
}
/// 查询指定时间范围地点版本的响应。
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
/// 查询指定时间范围地点版本的请求。
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    config: Arc<Config>,
    body: QueryMultiTimelineBody,
}
/// 查询指定时间范围地点版本请求构建器实现。
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
    /// 执行查询指定时间范围地点版本请求。
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        let request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/locations/query_multi_timeline",
        )
        .body(serde_json::to_value(&self.body)?);
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询指定时间范围地点版本", "响应数据为空")
        })
    }
}
