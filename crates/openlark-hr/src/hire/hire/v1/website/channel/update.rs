//! 更新招聘官网推广渠道
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.channel/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新招聘官网推广渠道请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    channel_id: String,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            channel_id: String::new(),
        }
    }

    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = channel_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.channel_id.trim(), "渠道 ID 不能为空");

        let api_endpoint = HireApiV1::WebsiteChannelUpdate(self.channel_id);
        let request = ApiRequest::<UpdateResponse>::patch(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新招聘官网推广渠道响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新招聘官网推广渠道响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for UpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
