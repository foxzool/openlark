//! 调用技能
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/app-skill/start>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SKILL_START};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 调用技能请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartSkillBody {
    pub input: serde_json::Value,
}

impl StartSkillBody {
    pub fn new(input: serde_json::Value) -> Self {
        Self { input }
    }
}

/// 调用技能请求
///
/// 用于调用指定的 Aily 技能。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
/// - `skill_id`: 技能 ID，必填
///
/// # 请求体字段
///
/// - `input`: 技能输入参数
///
/// # 示例
///
/// ```rust,ignore
/// let body = StartSkillBody::new(serde_json::json!({"query": "你好"}));
/// let request = StartSkillRequest::new(config)
///     .app_id("app_xxx")
///     .skill_id("skill_xxx")
///     .execute(body).await?;
/// ```
pub struct StartSkillRequest {
    config: Config,
    app_id: String,
    skill_id: String,
}

impl StartSkillRequest {
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
    /// docPath: <https://open.feishu.cn/document/aily-v1/app-skill/start>
    pub async fn execute(self, body: StartSkillBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: StartSkillBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.skill_id, "skill_id 不能为空");

        let url = AILY_V1_SKILL_START
            .replace("{app_id}", &self.app_id)
            .replace("{skill_id}", &self.skill_id);
        let req: ApiRequest<StartSkillBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "调用技能")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_start_skill_request_builder() {
        let config = Config::default();
        let request = StartSkillRequest::new(config)
            .app_id("app_xxx")
            .skill_id("skill_xxx");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.skill_id, "skill_xxx");
    }

    #[test]
    fn test_start_skill_body_builder() {
        let input = serde_json::json!({"query": "测试"});
        let body = StartSkillBody::new(input);
        assert_eq!(body.input["query"], "测试");
    }

    #[test]
    fn test_start_skill_request_default_values() {
        let config = Config::default();
        let request = StartSkillRequest::new(config);
        assert_eq!(request.app_id, "");
        assert_eq!(request.skill_id, "");
    }

    #[test]
    fn test_start_skill_request_chaining() {
        let config = Config::default();
        let request = StartSkillRequest::new(config)
            .app_id("app_123")
            .skill_id("skill_456");
        assert_eq!(request.app_id, "app_123");
        assert_eq!(request.skill_id, "skill_456");
    }

    #[test]
    fn test_start_skill_body_with_complex_input() {
        let input = serde_json::json!({
            "query": "问题",
            "context": ["上下文1", "上下文2"]
        });
        let body = StartSkillBody::new(input);
        assert!(body.input["query"].is_string());
    }
}
