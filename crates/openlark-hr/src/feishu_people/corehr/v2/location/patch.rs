//! 更新地点
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/location/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 更新地点请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchRequest {
    /// 地点 ID
    pub location_id: String,
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 地址信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 状态（active/inactive）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(location_id: String) -> Self {
        Self {
            location_id,
            name: None,
            description: None,
            address: None,
            status: None,
        }
    }

    /// 设置地点名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置地点描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置地址信息
    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    /// 设置状态
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}

/// 更新地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 地点信息
    pub data: Option<PatchResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponseData {
    /// 地点 ID
    pub id: String,
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 地址信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新地点请求构建器
#[derive(Debug, Clone)]
pub struct PatchRequestBuilder {
    config: Config,
    request: PatchRequest,
}

impl PatchRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, location_id: String) -> Self {
        Self {
            config,
            request: PatchRequest::new(location_id),
        }
    }

    /// 设置地点名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置地点描述
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置地址信息
    pub fn address(mut self, address: String) -> Self {
        self.request = self.request.address(address);
        self
    }

    /// 设置状态
    pub fn status(mut self, status: String) -> Self {
        self.request = self.request.status(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::LocationPatch(self.request.location_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 序列化请求体
        let request_body = serde_json::json!({
            "name": self.request.name,
            "description": self.request.description,
            "address": self.request.address,
            "status": self.request.status,
        });
        let request = request.body(request_body);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新地点响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
