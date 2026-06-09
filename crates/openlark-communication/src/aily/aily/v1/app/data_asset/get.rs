//! 获取数据知识
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/get

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSET};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

/// 获取数据知识请求
///
/// 用于获取指定数据知识的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
/// - `data_asset_id`: 数据知识 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetDataAssetRequest::new(config)
///     .app_id("app_xxx")
///     .data_asset_id("asset_xxx")
///     .execute().await?;
/// ```
pub struct GetDataAssetRequest {
    config: Config,
    app_id: String,
    data_asset_id: String,
}

impl GetDataAssetRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            data_asset_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 数据知识 ID（路径参数）
    pub fn data_asset_id(mut self, data_asset_id: impl Into<String>) -> Self {
        self.data_asset_id = data_asset_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.data_asset_id, "data_asset_id 不能为空");

        let url = AILY_V1_DATA_ASSET
            .replace("{app_id}", &self.app_id)
            .replace("{data_asset_id}", &self.data_asset_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取数据知识")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_asset_request_builder() {
        let config = Config::default();
        let request = GetDataAssetRequest::new(config)
            .app_id("app_xxx")
            .data_asset_id("asset_xxx");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.data_asset_id, "asset_xxx");
    }

    #[test]
    fn test_get_data_asset_request_default_values() {
        let config = Config::default();
        let request = GetDataAssetRequest::new(config);
        assert_eq!(request.app_id, "");
        assert_eq!(request.data_asset_id, "");
    }

    #[test]
    fn test_get_data_asset_request_with_app_id_only() {
        let config = Config::default();
        let request = GetDataAssetRequest::new(config).app_id("app_123");
        assert_eq!(request.app_id, "app_123");
        assert_eq!(request.data_asset_id, "");
    }

    #[test]
    fn test_get_data_asset_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_DATA_ASSET;
        let url = AILY_V1_DATA_ASSET
            .replace("{session_id}", "sess_1")
            .replace("{aily_session_id}", "sess_1")
            .replace("{run_id}", "run_1")
            .replace("{app_id}", "app_1")
            .replace("{data_asset_id}", "asset_1")
            .replace("{skill_id}", "skill_1")
            .replace("{message_id}", "msg_1");
        assert_eq!(url, "/open-apis/aily/v1/apps/app_1/data_assets/asset_1");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }

    #[test]
    fn test_get_data_asset_request_chaining() {
        let config = Config::default();
        let request = GetDataAssetRequest::new(config)
            .app_id("app_xxx")
            .data_asset_id("asset_xxx");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.data_asset_id, "asset_xxx");
    }
}
