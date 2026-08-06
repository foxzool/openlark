//! 全量更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>
//!
//! 请求体对齐官方 lark-oapi SDK（UpdateCardRequestBody：`card` + `uuid` + `sequence`）：
//! `{"card": {"type":"card_json","data":"..."}, "sequence": N}`。
//! 修正：原 `card_id/card_content` 格式与飞书实际 API 不符（未联调验证），
//! 且缺失必填 `sequence`（不带会报 300317 sequence compare failed）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{api_utils::serialize_params, validation::validate_card_id},
    endpoints::cardkit_v1_card,
};

/// 全量更新卡片实体请求体（官方格式：`card` + `uuid` + `sequence`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 卡片 ID（仅 URL path 用，不进请求体）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 卡片内容（`{"type":"card_json","data":"<json>"}`）
    pub card: serde_json::Value,
    /// 请求唯一标识（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 更新序号（递增；缺失可能报 300317 sequence compare failed）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
}

/// 全量更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardResponse {}

/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    card: Option<serde_json::Value>,
    uuid: Option<String>,
    sequence: Option<i64>,
}

impl UpdateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            card: None,
            uuid: None,
            sequence: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>
    pub async fn execute(self, body: UpdateCardBody) -> SDKResult<UpdateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>
    pub async fn execute_with_options(
        self,
        body: UpdateCardBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(card) = self.card {
            body.card = card;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }
        if let Some(sequence) = self.sequence {
            body.sequence = Some(sequence);
        }

        validate_card_id(&body.card_id)?;
        if body.card.is_null() || !body.card.is_object() {
            return Err(openlark_core::CoreError::validation_msg(
                "card 必须是 JSON 对象（{\"type\":\"card_json\",\"data\":...}）",
            ));
        }

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id
        let url = cardkit_v1_card(&body.card_id);
        let req: ApiRequest<UpdateCardResponse> =
            ApiRequest::put(url).body(serialize_params(&body, "全量更新卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "全量更新卡片实体").await
    }
}

/// 全量更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
    card_id: Option<String>,
    card: Option<serde_json::Value>,
    uuid: Option<String>,
    sequence: Option<i64>,
}

impl UpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardRequest::new(config),
            card_id: None,
            card: None,
            uuid: None,
            sequence: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置卡片内容
    pub fn card(mut self, card: impl Into<serde_json::Value>) -> Self {
        self.card = Some(card.into());
        self
    }

    /// 设置请求唯一标识
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 设置更新序号（递增；缺失可能报 300317）
    pub fn sequence(mut self, sequence: impl Into<i64>) -> Self {
        self.sequence = Some(sequence.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardRequest {
        UpdateCardRequest {
            config: self.request.config,
            card_id: self.card_id,
            card: self.card,
            uuid: self.uuid,
            sequence: self.sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_body_serializes_official_format_without_card_id() {
        let body = UpdateCardBody {
            card_id: "card_1".into(),
            card: serde_json::json!({"type": "card_json", "data": "{}"}),
            uuid: None,
            sequence: Some(3),
        };
        let json = serde_json::to_value(&body).unwrap();
        // 请求体含 card + sequence；card_id 不进请求体（URL path 用）
        assert_eq!(json["card"]["type"], "card_json");
        assert_eq!(json["sequence"], 3);
        assert!(json.get("card_id").is_none(), "card_id 不应出现在请求体");
    }
}
