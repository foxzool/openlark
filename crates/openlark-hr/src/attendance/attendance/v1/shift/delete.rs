//! 删除班次
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/shift/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::DeleteShiftResponse;

/// 删除班次请求
#[derive(Debug, Clone)]
pub struct DeleteShiftRequest {
    /// 班次 ID（必填）
    shift_id: String,
    /// 配置信息
    config: Config,
}

impl DeleteShiftRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            shift_id: String::new(),
            config,
        }
    }

    /// 设置班次 ID（必填）
    pub fn shift_id(mut self, shift_id: String) -> Self {
        self.shift_id = shift_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteShiftResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteShiftResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.shift_id.trim(), "班次 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ShiftDelete(self.shift_id.clone());
        let request = ApiRequest::<DeleteShiftResponse>::delete(api_endpoint.to_url());

        // 3. 发送请求（DELETE 无请求体）
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除班次响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for DeleteShiftResponse {
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
    fn test_delete_shift_request_builder_new() {
        let request =
            DeleteShiftRequest::new(TestConfigBuilder::new().build()).shift_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_delete_shift_request_validation_fails_on_default_request() {
        let request = DeleteShiftRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
