//! 新建记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新建记录 Builder
#[derive(Debug, Clone)]
pub struct RecordCreateBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录数据
    data: serde_json::Value,
}

impl RecordCreateBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            data: serde_json::json!({}),
        }
    }

    /// 设置记录数据
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = data.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordCreateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records",
            self.namespace, self.object_api_name
        );

        let request = RecordCreateRequest { data: self.data };

        let req: ApiRequest<RecordCreateResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 新建记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordCreateRequest {
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 新建记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordCreateResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
}

impl ApiResponseTrait for RecordCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request =
            RecordCreateBuilder::new(config.clone(), "test".to_string(), "test".to_string())
                .data(serde_json::json!({}));
        let _ = request;
    }
}
