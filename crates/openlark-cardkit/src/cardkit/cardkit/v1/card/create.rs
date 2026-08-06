//! 创建卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
//!
//! 请求体对齐官方 lark-oapi SDK（CreateCardRequestBody：`type` + `data` 平铺）：
//! `{"type": "card_json", "data": "<卡片 JSON 字符串>"}`。
//! 修正：原 `card_content` 包裹格式与飞书实际 API 不符（未联调验证）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::serialize_params, endpoints::CARDKIT_V1_CARDS};

/// 创建卡片实体请求体（官方格式：`type` + `data` 平铺）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardBody {
    /// 卡片类型（如 `card_json`）
    pub r#type: String,
    /// 卡片内容（卡片 JSON 字符串）
    pub data: String,
}

impl CreateCardBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if self.r#type.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("type 不能为空"));
        }
        if self.data.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("data 不能为空"));
        }
        Ok(())
    }
}

/// 创建卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateCardResponse {}

/// 创建卡片实体请求
#[derive(Debug, Clone)]
pub struct CreateCardRequest {
    config: Config,
}

impl CreateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute(self, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute_with_options(
        self,
        body: CreateCardBody,
        option: RequestOption,
    ) -> SDKResult<CreateCardResponse> {
        body.validate()?;

        // url: POST:/open-apis/cardkit/v1/cards
        let req: ApiRequest<CreateCardResponse> =
            ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "创建卡片实体").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_body_serializes_flat_official_format() {
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: r#"{"schema":"2.0"}"#.into(),
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["type"], "card_json");
        assert_eq!(json["data"], r#"{"schema":"2.0"}"#);
        // 官方平铺格式：无 card_content 包裹
        assert!(json.get("card_content").is_none());
        assert!(body.validate().is_ok());
    }

    #[test]
    fn create_body_validation_rejects_empty() {
        let body = CreateCardBody {
            r#type: String::new(),
            data: "{}".into(),
        };
        assert!(body.validate().is_err());
    }
}
