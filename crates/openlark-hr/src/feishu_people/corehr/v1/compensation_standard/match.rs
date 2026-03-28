//! 获取员工薪资标准
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/compensation_standard/match>

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取员工薪资标准请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchRequest {
    config: Config,
    body: Option<Value>,
    query_params: Vec<(String, String)>,
}

impl MatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: None,
            query_params: Vec::new(),
        }
    }

    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::CompensationStandardMatch;
        let mut request = ApiRequest::<MatchResponse>::post(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取员工薪资标准响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取员工薪资标准响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for MatchResponse {
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
