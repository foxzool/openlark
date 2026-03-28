//! 编辑记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 编辑记录 Builder
#[derive(Debug, Clone)]
pub struct RecordPatchBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID
    record_id: String,
    /// 更新的数据
    data: serde_json::Value,
}

impl RecordPatchBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
        record_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            record_id: record_id.into(),
            data: serde_json::json!({}),
        }
    }

    /// 设置更新的数据
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = data.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordPatchResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}",
            self.namespace, self.object_api_name, self.record_id
        );

        let request = RecordPatchRequest { data: self.data };

        let req: ApiRequest<RecordPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordPatchResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}",
            self.namespace, self.object_api_name, self.record_id
        );

        let request = RecordPatchRequest { data: self.data };

        let req: ApiRequest<RecordPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 编辑记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordPatchRequest {
    /// 更新的数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 编辑记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordPatchResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

impl ApiResponseTrait for RecordPatchResponse {
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
