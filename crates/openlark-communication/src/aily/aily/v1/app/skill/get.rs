//! 获取技能信息
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/app-skill/get>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SKILL};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 获取技能信息请求
///
/// 用于获取指定技能的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
/// - `skill_id`: 技能 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetSkillRequest::new(config)
///     .app_id("app_xxx")
///     .skill_id("skill_xxx")
///     .execute().await?;
/// ```
pub struct GetSkillRequest {
    config: Config,
    app_id: String,
    skill_id: String,
}

impl GetSkillRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            skill_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 技能 ID（路径参数）
    pub fn skill_id(mut self, skill_id: impl Into<String>) -> Self {
        self.skill_id = skill_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/app-skill/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.skill_id, "skill_id 不能为空");

        let url = AILY_V1_SKILL
            .replace("{app_id}", &self.app_id)
            .replace("{skill_id}", &self.skill_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取技能信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_skill_request_builder() {
        let config = Config::default();
        let request = GetSkillRequest::new(config)
            .app_id("app_xxx")
            .skill_id("skill_xxx");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.skill_id, "skill_xxx");
    }

    #[test]
    fn test_get_skill_request_default_values() {
        let config = Config::default();
        let request = GetSkillRequest::new(config);
        assert_eq!(request.app_id, "");
        assert_eq!(request.skill_id, "");
    }

    #[test]
    fn test_get_skill_request_with_app_id_only() {
        let config = Config::default();
        let request = GetSkillRequest::new(config).app_id("app_123");
        assert_eq!(request.app_id, "app_123");
        assert_eq!(request.skill_id, "");
    }

    #[test]
    fn test_get_skill_request_url_construction() {
        let request = GetSkillRequest::new(Config::default())
            .app_id("app_1")
            .skill_id("skill_1");
        assert_eq!(request.app_id, "app_1");
        assert_eq!(request.skill_id, "skill_1");
    }

    #[test]
    fn test_get_skill_request_chaining() {
        let config = Config::default();
        let request = GetSkillRequest::new(config)
            .app_id("app_xxx")
            .skill_id("skill_xxx");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.skill_id, "skill_xxx");
    }
}
