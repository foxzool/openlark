//! 更新人才在职状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/onboard_status>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 更新人才在职状态请求
#[derive(Debug, Clone)]
pub struct OnboardStatusRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
    request_body: Option<Value>,
}

impl OnboardStatusRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `talent_id`。
    pub fn talent_id(mut self, talent_id: impl Into<String>) -> Self {
        self.talent_id = talent_id.into();
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OnboardStatusResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OnboardStatusResponse> {
        validate_required!(self.talent_id.trim(), "talent_id 不能为空");

        let mut request = ApiRequest::<OnboardStatusResponse>::post(format!(
            "/open-apis/hire/v1/talents/{}/onboard_status",
            self.talent_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新人才在职状态响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新人才在职状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OnboardStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_status` 字段。
    pub onboard_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for OnboardStatusResponse {
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
    fn test_onboard_status_request_builder_new() {
        let request = OnboardStatusRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
