//! 获取职位列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::models::JobDetail;

/// 获取职位列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    page_size: Option<i32>,
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::JobList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位列表响应数据为空",
        )
        .await
    }
}

/// 获取职位列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<JobDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn test_list_request_builder_new() {
        let request = ListRequest::new(TestConfigBuilder::new().build()).page_size(1);
        let _ = request;
    }

    /// 端到端：GET /open-apis/hire/v1/jobs，响应 items 元素按官方 job 形状（id/title/description）解析。
    #[tokio::test]
    async fn test_list_parses_official_job_shape() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/jobs"))
            .and(query_param("page_size", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "items": [
                        {
                            "id": "7379609334215919883",
                            "title": "测试职位",
                            "description": "这是一个测试职位",
                            "code": "R18",
                            "requirement": "要求应届生",
                            "recruitment_type": {
                                "id": "7379609334215919883",
                                "zh_name": "全职",
                                "en_name": "FullTime",
                                "active_status": 1
                            },
                            "department": {
                                "id": "od-xxxx",
                                "zh_name": "字节跳动",
                                "en_name": "Bytedance"
                            },
                            "active_status": 1,
                            "process_type": 1,
                            "process_id": "1",
                            "process_name": "校招招聘流程"
                        }
                    ],
                    "has_more": false
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

        let resp = ListRequest::new(config)
            .page_size(10)
            .execute()
            .await
            .expect("请求应成功");

        let job = resp.items.first().expect("应解析出 1 个职位");
        assert_eq!(job.id.as_deref(), Some("7379609334215919883"));
        assert_eq!(job.title.as_deref(), Some("测试职位"));
        assert_eq!(job.description.as_deref(), Some("这是一个测试职位"));
        assert_eq!(
            job.recruitment_type
                .as_ref()
                .and_then(|r| r.zh_name.as_deref()),
            Some("全职")
        );
        assert_eq!(
            job.department.as_ref().and_then(|d| d.en_name.as_deref()),
            Some("Bytedance")
        );
    }
}
