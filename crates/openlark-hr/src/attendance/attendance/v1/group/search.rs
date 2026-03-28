//! 按名称查询考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::SearchGroupResponse;

/// 按名称查询考勤组请求
#[derive(Debug, Clone)]
pub struct SearchGroupRequest {
    /// 考勤组名称（必填）
    group_name: String,
    /// 分页大小，默认 10，最大 100
    page_size: Option<i32>,
    /// 分页标记，用于获取下一页数据
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl SearchGroupRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_name: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置考勤组名称（必填）
    pub fn group_name(mut self, group_name: String) -> Self {
        self.group_name = group_name;
        self
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
    pub async fn execute(self) -> SDKResult<SearchGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_name.trim(), "考勤组名称不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::GroupSearch;
        let mut request = ApiRequest::<SearchGroupResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("group_name", &self.group_name);
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "搜索考勤组响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for SearchGroupResponse {
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
    fn test_search_group_request_builder_new() {
        let request = SearchGroupRequest::new(TestConfigBuilder::new().build())
            .group_name("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_search_group_request_validation_fails_on_default_request() {
        let request = SearchGroupRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
