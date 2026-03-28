//! 获取候选人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{GetRequestBody, GetResponse};

/// 获取候选人信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 候选人 ID（必填）
    talent_id: String,
    /// 是否需要详细信息
    need_detail: Option<bool>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
            need_detail: None,
        }
    }

    /// 设置候选人 ID（必填）
    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 设置是否需要详细信息
    pub fn need_detail(mut self, need_detail: bool) -> Self {
        self.need_detail = Some(need_detail);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 1. 验证必填字段
        validate_required!(self.talent_id.trim(), "候选人 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = HireApiV1::TalentGet(self.talent_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = GetRequestBody {
            need_detail: self.need_detail,
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
                "获取候选人信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for GetResponse {
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
    fn test_get_request_builder_new() {
        let request =
            GetRequest::new(TestConfigBuilder::new().build()).talent_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_request_validation_fails_on_default_request() {
        let request = GetRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
