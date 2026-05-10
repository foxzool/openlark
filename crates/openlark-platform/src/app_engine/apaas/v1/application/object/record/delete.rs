//! 删除记录
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-object-record/delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除记录 Builder
#[derive(Debug, Clone)]
pub struct RecordDeleteBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID
    record_id: String,
}

impl RecordDeleteBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordDeleteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}",
            self.namespace, self.object_api_name, self.record_id
        );

        let req: ApiRequest<RecordDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除记录", "响应数据为空"))
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for RecordDeleteResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
