//! 查询多时间轴
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{MultiTimelineRequestBody, MultiTimelineResponse};

/// 查询多时间轴请求
#[derive(Debug, Clone)]
pub struct MultiTimelineRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID 列表（必填）
    department_ids: Vec<String>,
    /// 开始时间（必填，格式：YYYY-MM-DD）
    start_time: String,
    /// 结束时间（必填，格式：YYYY-MM-DD）
    end_time: String,
}

impl MultiTimelineRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_ids: Vec::new(),
            start_time: String::new(),
            end_time: String::new(),
        }
    }

    /// 设置部门 ID 列表（必填）
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = department_ids;
        self
    }

    /// 设置开始时间（必填，格式：YYYY-MM-DD）
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = start_time;
        self
    }

    /// 设置结束时间（必填，格式：YYYY-MM-DD）
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = end_time;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MultiTimelineResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.department_ids, "部门 ID 列表不能为空");
        validate_required!(self.start_time.trim(), "开始时间不能为空");
        validate_required!(self.end_time.trim(), "结束时间不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentQueryMultiTimeline;
        let request = ApiRequest::<MultiTimelineResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = MultiTimelineRequestBody {
            department_ids: self.department_ids,
            start_time: self.start_time,
            end_time: self.end_time,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询多时间轴响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for MultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/departments/query_multi_timeline
    #[tokio::test]
    async fn test_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v1/departments/query_multi_timeline",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "items": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        MultiTimelineRequest::new(config)
            .department_ids(vec!["test001".to_string()])
            .start_time("test001".to_string())
            .end_time("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
