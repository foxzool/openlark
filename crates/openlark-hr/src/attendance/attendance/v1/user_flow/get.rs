//! 获取打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{GetUserFlowRequestBody, GetUserFlowResponse};

/// 获取打卡流水请求
#[derive(Debug, Clone)]
pub struct GetUserFlowRequest {
    /// 打卡流水 ID
    user_flow_id: String,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl GetUserFlowRequest {
    /// 创建获取打卡流水请求
    pub fn new(config: Config) -> Self {
        Self {
            user_flow_id: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置打卡流水 ID（必填）
    pub fn user_flow_id(mut self, user_flow_id: String) -> Self {
        self.user_flow_id = user_flow_id;
        self
    }

    /// 设置用户 ID 类型（可选）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_flow_id.trim(), "打卡流水 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowGet;
        let mut request = ApiRequest::<GetUserFlowResponse>::post(api_endpoint.to_url());

        // 3. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 4. 序列化请求体
        let request_body = GetUserFlowRequestBody {
            user_flow_id: self.user_flow_id,
            user_id_type: self.user_id_type,
        };
        request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 5. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 6. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取打卡流水响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for GetUserFlowResponse {
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
    fn test_get_user_flow_request_builder_new() {
        let request = GetUserFlowRequest::new(TestConfigBuilder::new().build())
            .user_flow_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_user_flow_request_validation_fails_on_default_request() {
        let request = GetUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
