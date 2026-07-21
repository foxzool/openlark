//! 批量查询任职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/job_data/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use super::models::{ListRequestBody, ListResponse};

/// 批量查询任职信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（1-100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 员工 ID 过滤
    employee_id: Option<String>,
    /// 状态过滤
    status: Option<i32>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            employee_id: None,
            status: None,
        }
    }

    /// 设置分页大小（1-100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置员工 ID 过滤
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = Some(employee_id);
        self
    }

    /// 设置状态过滤
    /// - 1: 在职
    /// - 2: 离职
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证分页参数
        if let Some(size) = self.page_size
            && !(1..=100).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页参数无效",
                format!("分页大小必须在 1-100 之间，当前值为: {size}"),
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::JobDataList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 序列化请求体（作为查询参数）
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
            employee_id: self.employee_id,
            status: self.status,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询任职信息响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/job_datas
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/job_datas"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_data_list": [] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
