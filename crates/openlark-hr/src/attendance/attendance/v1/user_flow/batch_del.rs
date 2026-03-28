//! 删除打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/batch_del>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, SDKResult,
};

use super::models::{BatchDelUserFlowRequestBody, BatchDelUserFlowResponse};

/// 删除打卡流水请求
#[derive(Debug, Clone)]
pub struct BatchDelUserFlowRequest {
    /// 要删除的打卡流水 ID 列表
    user_flow_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl BatchDelUserFlowRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            user_flow_ids: Vec::new(),
            config,
        }
    }

    /// 添加要删除的打卡流水 ID
    pub fn add_user_flow_id(mut self, user_flow_id: String) -> Self {
        self.user_flow_ids.push(user_flow_id);
        self
    }

    /// 设置要删除的打卡流水 ID 列表
    pub fn user_flow_ids(mut self, user_flow_ids: Vec<String>) -> Self {
        self.user_flow_ids = user_flow_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDelUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDelUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.user_flow_ids,
            100,
            "打卡流水 ID 列表不能为空且不能超过 100 个"
        );
        for (idx, flow_id) in self.user_flow_ids.iter().enumerate() {
            validate_required!(
                flow_id.trim(),
                &format!("第 {} 个打卡流水 ID 不能为空", idx + 1)
            );
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowBatchDel;
        let request = ApiRequest::<BatchDelUserFlowResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchDelUserFlowRequestBody {
            user_flow_ids: self.user_flow_ids,
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
                "删除打卡流水响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchDelUserFlowResponse {
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
    fn test_batch_del_user_flow_request_builder_new() {
        let request = BatchDelUserFlowRequest::new(TestConfigBuilder::new().build())
            .add_user_flow_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_batch_del_user_flow_request_validation_fails_on_default_request() {
        let request = BatchDelUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
