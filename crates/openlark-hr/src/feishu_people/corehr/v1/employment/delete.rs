//! 删除雇佣信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/employment/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{DeleteRequestBody, DeleteResponse};

/// 删除雇佣信息请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    /// 雇佣 ID（必填）
    employment_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employment_id: String::new(),
        }
    }

    /// 设置雇佣 ID（必填）
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.employment_id = employment_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.employment_id.trim(), "雇佣 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::EmploymentDelete(self.employment_id.clone());
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = DeleteRequestBody {
            employment_id: self.employment_id,
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
                "删除雇佣信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for DeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_delete_request_builder_new() {
        let request =
            DeleteRequest::new(TestConfigBuilder::new().build()).employment_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_delete_request_validation_fails_on_default_request() {
        let request = DeleteRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
