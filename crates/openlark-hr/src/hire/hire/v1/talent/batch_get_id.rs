//! 批量获取候选人ID
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/batch_get_id>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list, SDKResult,
};

use super::models::{BatchGetIdRequestBody, BatchGetIdResponse};

/// 批量获取候选人ID请求
#[derive(Debug, Clone)]
pub struct BatchGetIdRequest {
    /// 配置信息
    config: Config,
    /// 候选人 ID 列表（必填，最多100个）
    talent_ids: Vec<String>,
}

impl BatchGetIdRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_ids: Vec::new(),
        }
    }

    /// 设置候选人 ID 列表（必填，最多100个）
    pub fn talent_ids(mut self, talent_ids: Vec<String>) -> Self {
        self.talent_ids = talent_ids;
        self
    }

    /// 添加单个候选人 ID
    pub fn add_talent_id(mut self, talent_id: String) -> Self {
        self.talent_ids.push(talent_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetIdResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetIdResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.talent_ids,
            100,
            "候选人 ID 列表不能为空且不能超过 100 个"
        );

        // 2. 构建端点
        let api_endpoint = HireApiV1::TalentBatchGetId;
        let request = ApiRequest::<BatchGetIdResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchGetIdRequestBody {
            talent_ids: self.talent_ids,
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
                "批量获取候选人ID响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchGetIdResponse {
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
    fn test_batch_get_id_request_builder_new() {
        let request = BatchGetIdRequest::new(TestConfigBuilder::new().build())
            .talent_ids(vec!["test".to_string()]);
        let _ = request;
    }
}
