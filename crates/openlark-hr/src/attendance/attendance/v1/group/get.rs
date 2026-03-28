//! 按 ID 查询考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::GetGroupResponse;

/// 按 ID 查询考勤组请求
#[derive(Debug, Clone)]
pub struct GetGroupRequest {
    /// 考勤组 ID（必填）
    group_id: String,
    /// 配置信息
    config: Config,
}

impl GetGroupRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_id: String::new(),
            config,
        }
    }

    /// 设置考勤组 ID（必填）
    pub fn group_id(mut self, group_id: String) -> Self {
        self.group_id = group_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_id.trim(), "考勤组 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::GroupGet(self.group_id.clone());
        let request = ApiRequest::<GetGroupResponse>::get(api_endpoint.to_url());

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取考勤组响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for GetGroupResponse {
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
    fn test_get_group_request_builder_new() {
        let request =
            GetGroupRequest::new(TestConfigBuilder::new().build()).group_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_group_request_validation_fails_on_default_request() {
        let request = GetGroupRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
