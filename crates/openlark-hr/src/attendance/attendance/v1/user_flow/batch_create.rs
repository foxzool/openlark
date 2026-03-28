//! 导入打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/batch_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list, SDKResult,
};

use super::models::{BatchCreateUserFlowRequestBody, BatchCreateUserFlowResponse, UserFlowRecord};

/// 导入打卡流水请求
#[derive(Debug, Clone)]
pub struct BatchCreateUserFlowRequest {
    /// 打卡流水记录列表
    flow_records: Vec<UserFlowRecord>,
    /// 配置信息
    config: Config,
}

impl BatchCreateUserFlowRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            flow_records: Vec::new(),
            config,
        }
    }

    /// 添加打卡流水记录
    pub fn add_flow_record(mut self, record: UserFlowRecord) -> Self {
        self.flow_records.push(record);
        self
    }

    /// 设置打卡流水记录列表
    pub fn flow_records(mut self, records: Vec<UserFlowRecord>) -> Self {
        self.flow_records = records;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.flow_records,
            100,
            "打卡流水记录列表不能为空且不能超过 100 个"
        );
        for (idx, record) in self.flow_records.iter().enumerate() {
            validate_required!(
                record.user_id.trim(),
                &format!("第 {} 条记录的用户 ID 不能为空", idx + 1)
            );
            validate_required!(
                record.punch_time.trim(),
                &format!("第 {} 条记录的打卡时间不能为空", idx + 1)
            );
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowBatchCreate;
        let request = ApiRequest::<BatchCreateUserFlowResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchCreateUserFlowRequestBody {
            flow_records: self.flow_records,
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
                "导入打卡流水响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchCreateUserFlowResponse {
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
    fn test_batch_create_user_flow_request_builder_new() {
        let request = BatchCreateUserFlowRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }

    #[test]
    fn test_batch_create_user_flow_request_validation_fails_on_default_request() {
        let request = BatchCreateUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
