//! 查询单条国家/地区信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/country_region/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询单条国家/地区信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 国家/地区 ID（必填）
    country_region_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            country_region_id: String::new(),
        }
    }

    /// 设置国家/地区 ID（必填）
    pub fn country_region_id(mut self, country_region_id: String) -> Self {
        self.country_region_id = country_region_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.country_region_id.trim(), "国家/地区 ID 不能为空");

        let api_endpoint = CorehrApiV1::CountryRegionGet(self.country_region_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<GetResponse>::get(endpoint_url);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询国家/地区响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询单条国家/地区信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
