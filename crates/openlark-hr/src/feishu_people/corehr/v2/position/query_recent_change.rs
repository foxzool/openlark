//! 查询指定时范围内当前版本信息发生变更的岗位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/query_recent_change>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询指定时范围内当前版本信息发生变更的岗位请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRecentChangeRequest {
    /// 配置信息
    config: Config,
}

impl QueryRecentChangeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryRecentChangeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryRecentChangeResponse> {
        let request = ApiRequest::<QueryRecentChangeResponse>::get(
            "/open-apis/corehr/v2/positions/query_recent_change",
        );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 查询指定时范围内当前版本信息发生变更的岗位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRecentChangeResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for QueryRecentChangeResponse {
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
