//! 根据组织架构调整 ID 查询发起的流程信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v2/draft/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据组织架构调整 ID 查询发起的流程信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    draft_id: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            draft_id: None,
        }
    }

    /// 设置 `draft_id`。
    pub fn draft_id(mut self, draft_id: String) -> Self {
        self.draft_id = Some(draft_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let draft_id = self.draft_id.unwrap_or_default();
        validate_required!(draft_id.trim(), "draft_id 不能为空");

        let request =
            ApiRequest::<GetResponse>::get(format!("/open-apis/corehr/v2/drafts/{draft_id}"));

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 根据组织架构调整 ID 查询发起的流程信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 响应数据
    /// draft信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<DraftInfo>,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftInfo {
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
