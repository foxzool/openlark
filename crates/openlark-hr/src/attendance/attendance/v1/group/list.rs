//! 查询所有考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::ListGroupResponse;

/// 查询所有考勤组请求
#[derive(Debug, Clone)]
pub struct ListGroupRequest {
    /// 分页大小，默认 10，最大 100
    page_size: Option<i32>,
    /// 分页标记，用于获取下一页数据
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListGroupRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::GroupList;
        let mut request = ApiRequest::<ListGroupResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询考勤组列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for ListGroupResponse {
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
    fn test_list_group_request_builder_new() {
        let request = ListGroupRequest::new(TestConfigBuilder::new().build()).page_size(1);
        let _ = request;
    }
}
