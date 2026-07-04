//! 编辑关键结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/key_result/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 编辑关键结果请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    key_result_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            key_result_id: String::new(),
        }
    }

    /// 设置路径参数 `key_result_id`。
    pub fn key_result_id(mut self, val: impl Into<String>) -> Self {
        self.key_result_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchKeyResultResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<PatchKeyResultResponse> {
        validate_required!(self.key_result_id, "key_result_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = format!("/open-apis/okr/v2/key_results/{}", self.key_result_id);
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<PatchKeyResultResponse> = ApiRequest::patch(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("编辑关键结果", "响应数据为空"))
    }
}

/// 编辑关键结果响应。
#[derive(Debug, Clone, Deserialize)]
pub struct PatchKeyResultResponse {
    /// 关键结果详情。
    #[serde(default)]
    pub key_result: Option<KeyResult>,
}

impl ApiResponseTrait for PatchKeyResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 关键结果。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResult {
    /// 关键结果的 ID。
    pub id: String,
    /// 关键结果的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 关键结果的修改时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: KeyResultOwner,
    /// 关键结果的目标 ID。
    pub objective_id: String,
    /// 关键结果的序号：从 1 开始计数。
    pub position: i32,
    /// 关键结果的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 关键结果的分数：[0,1]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的权重：[0,1]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 关键结果的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
}

/// 关键结果所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_patch_key_result_response_deserialize() {
        let json = serde_json::json!({
            "key_result": {
                "id": "KR-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "objective_id": "O-123",
                "position": 1,
                "score": 0.8,
                "weight": 0.5,
                "deadline": "1700000000000"
            }
        });
        let resp: PatchKeyResultResponse = serde_json::from_value(json).expect("反序列化失败");
        let kr = resp.key_result.expect("关键结果不应为空");
        assert_eq!(kr.id, "KR-123");
        assert_eq!(kr.objective_id, "O-123");
        assert_eq!(kr.position, 1);
        assert_eq!(kr.score, Some(0.8));
    }

    #[test]
    fn test_patch_key_result_response_deserialize_empty() {
        let json = serde_json::json!({});
        let resp: PatchKeyResultResponse = serde_json::from_value(json).expect("反序列化失败");
        assert!(resp.key_result.is_none());
    }
}
