//! 获取工作空间下的自定义枚举列表
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/enums
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-enum/list
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/enums

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取枚举列表 Builder
#[derive(Debug, Clone)]
pub struct EnumListBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
}

impl EnumListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnumListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EnumListResponse> {
        let url = format!("/open-apis/apaas/v1/workspaces/{}/enums", self.workspace_id);

        let req: ApiRequest<EnumListResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 枚举信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumInfo {
    /// 枚举名称
    #[serde(rename = "enum_name")]
    enum_name: String,
    /// 枚举描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 枚举值列表
    #[serde(rename = "values")]
    values: Vec<EnumValue>,
}

/// 枚举值
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumValue {
    /// 值 ID
    #[serde(rename = "value_id")]
    value_id: String,
    /// 值名称
    #[serde(rename = "value_name")]
    value_name: String,
    /// 是否为默认值
    #[serde(rename = "is_default")]
    is_default: bool,
}

/// 枚举列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumListResponse {
    /// 枚举列表
    #[serde(rename = "items")]
    items: Vec<EnumInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for EnumListResponse {
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
        let request = EnumListBuilder::new(config.clone(), "test".to_string());
        let _ = request;
    }
}
