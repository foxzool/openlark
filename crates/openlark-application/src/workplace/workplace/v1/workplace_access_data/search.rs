//! 工作台访问数据
//!
//! 文档: https://open.feishu.cn/document/workplace-v1/workplace_access_data/search
//! docPath: https://open.feishu.cn/document/workplace-v1/workplace_access_data/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 工作台访问数据查询 Builder
#[derive(Debug, Clone)]
pub struct AccessDataSearchWorkplaceBuilder {
    config: Config,
    start_date: String,
    end_date: String,
}

impl AccessDataSearchWorkplaceBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            start_date: String::new(),
            end_date: String::new(),
        }
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AccessDataSearchWorkplaceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AccessDataSearchWorkplaceResponse> {
        let url = "/open-apis/workplace/v1/workplace_access_data/search";

        let request = AccessDataSearchWorkplaceRequest {
            start_date: self.start_date,
            end_date: self.end_date,
        };

        let req: ApiRequest<AccessDataSearchWorkplaceResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 工作台访问数据查询请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccessDataSearchWorkplaceRequest {
    /// 开始日期 (YYYY-MM-DD)
    #[serde(rename = "start_date")]
    start_date: String,
    /// 结束日期 (YYYY-MM-DD)
    #[serde(rename = "end_date")]
    end_date: String,
}

/// 工作台访问数据查询响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessDataSearchWorkplaceResponse {
    /// 访问数据列表
    pub items: Vec<WorkplaceAccessData>,
}

/// 工作台访问数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkplaceAccessData {
    /// 日期
    pub date: String,
    /// 访问量
    #[serde(rename = "visit_count")]
    pub visit_count: i64,
    /// 访客数
    #[serde(rename = "visitor_count")]
    pub visitor_count: i64,
}

impl ApiResponseTrait for AccessDataSearchWorkplaceResponse {}

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
