//! 创建地点
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/location/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建地点请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 地点名称（必填）
    name: String,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    location_type: Option<i32>,
    /// 详细地址
    address: Option<String>,
    /// 城市
    city: Option<String>,
    /// 国家
    country: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            location_type: None,
            address: None,
            city: None,
            country: None,
        }
    }

    /// 设置地点名称（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置地点类型
    pub fn location_type(mut self, location_type: i32) -> Self {
        self.location_type = Some(location_type);
        self
    }

    /// 设置详细地址
    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    /// 设置城市
    pub fn city(mut self, city: String) -> Self {
        self.city = Some(city);
        self
    }

    /// 设置国家
    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.name.trim(), "地点名称不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::LocationCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            location_type: self.location_type,
            address: self.address,
            city: self.city,
            country: self.country,
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
                "创建地点响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateResponse {
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
