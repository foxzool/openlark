//! 获取候选人列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::{ListRequestBody, ListResponse};

/// 获取候选人列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（1-100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 查询关键词（姓名、邮箱、手机号）
    query: Option<String>,
    /// 候选人状态列表
    status_list: Option<Vec<i32>>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            query: None,
            status_list: None,
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

    /// 设置查询关键词
    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    /// 设置候选人状态列表
    pub fn status_list(mut self, status_list: Vec<i32>) -> Self {
        self.status_list = Some(status_list);
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
        use crate::common::api_endpoints::HireApiV1;

        // 1. 验证分页大小范围
        if let Some(size) = self.page_size {
            if !(1..=100).contains(&size) {
                return Err(openlark_core::error::validation_error(
                    "分页大小超出范围",
                    "page_size 必须在 1-100 之间",
                ));
            }
        }

        // 2. 构建端点
        let api_endpoint = HireApiV1::TalentList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
            query: self.query,
            status_list: self.status_list,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取候选人列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
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

    #[test]
    fn test_list_request_builder_new() {
        let request = ListRequest::new(TestConfigBuilder::new().build()).page_size(1);
        let _ = request;
    }
}
