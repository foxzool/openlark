//! 获取关联组织双方共享成员范围
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list-3
//! docPath: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list-3

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织双方共享成员范围 Builder
#[derive(Debug, Clone)]
pub struct CollaborationShareEntityListBuilder {
    config: Config,
    /// 租户 ID
    tenant_id: Option<String>,
}

impl CollaborationShareEntityListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tenant_id: None,
        }
    }

    /// 设置租户 ID
    pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationShareEntityListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationShareEntityListResponse> {
        let url = "/open-apis/directory/v1/share_entities".to_string();

        let mut req: ApiRequest<CollaborationShareEntityListResponse> = ApiRequest::get(&url);
        if let Some(tenant_id) = &self.tenant_id {
            req = req.query("tenant_id", tenant_id);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取共享成员", "响应数据为空"))
    }
}

/// 共享成员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareEntityInfo {
    /// 实体 ID
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// 实体类型
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// 实体名称
    #[serde(rename = "entity_name")]
    pub entity_name: String,
}

/// 获取关联组织双方共享成员范围响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationShareEntityListResponse {
    /// 共享成员列表
    #[serde(rename = "items")]
    pub items: Vec<ShareEntityInfo>,
}

impl ApiResponseTrait for CollaborationShareEntityListResponse {
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
            CollaborationShareEntityListBuilder::new(config.clone()).tenant_id("test".to_string());
        let _ = request;
    }
}
