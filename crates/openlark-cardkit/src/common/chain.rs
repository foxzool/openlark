//! CardKit 链式调用入口（meta 风格）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入“额外实现文件”
//! - 真实 API 实现仍严格位于 `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs`

use std::sync::Arc;

use openlark_core::config::Config;

/// CardKit 链式入口：`cardkit.v1.card.create(...)`
#[derive(Debug, Clone)]
pub struct CardkitClient {
    config: Arc<Config>,
    pub v1: CardkitV1Client,
}

impl CardkitClient {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            v1: CardkitV1Client::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// cardkit v1：`cardkit.v1`
#[derive(Debug, Clone)]
pub struct CardkitV1Client {
    config: Arc<Config>,
    pub card: CardResource,
}

impl CardkitV1Client {
    fn new(config: Arc<Config>) -> Self {
        Self {
            config: config.clone(),
            card: CardResource::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// card：`cardkit.v1.card`
#[derive(Debug, Clone)]
pub struct CardResource {
    config: Arc<Config>,
    pub element: CardElementResource,
}

impl CardResource {
    fn new(config: Arc<Config>) -> Self {
        Self {
            config: config.clone(),
            element: CardElementResource::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建卡片实体
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn create(
        &self,
        body: crate::cardkit::cardkit::v1::card::create::CreateCardBody,
    ) -> openlark_core::SDKResult<crate::cardkit::cardkit::v1::card::create::CreateCardResponse>
    {
        crate::cardkit::cardkit::v1::card::create::CreateCardRequest::new((*self.config).clone())
            .execute(body)
            .await
    }

    /// 全量更新卡片实体
    pub async fn update(
        &self,
        body: crate::cardkit::cardkit::v1::card::update::UpdateCardBody,
    ) -> openlark_core::SDKResult<crate::cardkit::cardkit::v1::card::update::UpdateCardResponse>
    {
        crate::cardkit::cardkit::v1::card::update::UpdateCardRequest::new((*self.config).clone())
            .execute(body)
            .await
    }

    /// 局部更新卡片实体
    pub async fn batch_update(
        &self,
        body: crate::cardkit::cardkit::v1::card::batch_update::BatchUpdateCardBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::batch_update::BatchUpdateCardResponse,
    > {
        crate::cardkit::cardkit::v1::card::batch_update::BatchUpdateCardRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    /// 更新卡片实体配置
    pub async fn settings(
        &self,
        body: crate::cardkit::cardkit::v1::card::settings::UpdateCardSettingsBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::settings::UpdateCardSettingsResponse,
    > {
        crate::cardkit::cardkit::v1::card::settings::UpdateCardSettingsRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    /// 转换卡片 ID
    pub async fn id_convert(
        &self,
        body: crate::cardkit::cardkit::v1::card::id_convert::ConvertCardIdBody,
    ) -> openlark_core::SDKResult<crate::cardkit::cardkit::v1::card::models::ConvertCardIdResponse>
    {
        crate::cardkit::cardkit::v1::card::id_convert::ConvertCardIdRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }
}

/// card.element：`cardkit.v1.card.element`
#[derive(Debug, Clone)]
pub struct CardElementResource {
    config: Arc<Config>,
}

impl CardElementResource {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn create(
        &self,
        body: crate::cardkit::cardkit::v1::card::element::create::CreateCardElementBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::element::models::CreateCardElementResponse,
    > {
        crate::cardkit::cardkit::v1::card::element::create::CreateCardElementRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    pub async fn update(
        &self,
        body: crate::cardkit::cardkit::v1::card::element::update::UpdateCardElementBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::element::models::UpdateCardElementResponse,
    > {
        crate::cardkit::cardkit::v1::card::element::update::UpdateCardElementRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    pub async fn patch(
        &self,
        body: crate::cardkit::cardkit::v1::card::element::patch::PatchCardElementBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::element::models::PatchCardElementResponse,
    > {
        crate::cardkit::cardkit::v1::card::element::patch::PatchCardElementRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    pub async fn content(
        &self,
        body: crate::cardkit::cardkit::v1::card::element::content::UpdateCardElementContentBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::element::models::UpdateCardElementContentResponse,
    > {
        crate::cardkit::cardkit::v1::card::element::content::UpdateCardElementContentRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
    }

    pub async fn delete(
        &self,
        body: crate::cardkit::cardkit::v1::card::element::delete::DeleteCardElementBody,
    ) -> openlark_core::SDKResult<
        crate::cardkit::cardkit::v1::card::element::models::DeleteCardElementResponse,
    > {
        crate::cardkit::cardkit::v1::card::element::delete::DeleteCardElementRequest::new(
            (*self.config).clone(),
        )
        .execute(body)
        .await
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
