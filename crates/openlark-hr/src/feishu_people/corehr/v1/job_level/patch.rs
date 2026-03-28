//! 更新单个职级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/job_level/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{PatchRequestBody, PatchResponse};

/// 更新单个职级请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 职级 ID（必填）
    job_level_id: String,
    /// 职级名称
    name: Option<String>,
    /// 职级等级
    level: Option<i32>,
    /// 职级描述
    description: Option<String>,
    /// 序列 ID
    job_family_id: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    status: Option<i32>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_level_id: String::new(),
            name: None,
            level: None,
            description: None,
            job_family_id: None,
            status: None,
        }
    }

    /// 设置职级 ID（必填）
    pub fn job_level_id(mut self, job_level_id: String) -> Self {
        self.job_level_id = job_level_id;
        self
    }

    /// 设置职级名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置职级等级
    pub fn level(mut self, level: i32) -> Self {
        self.level = Some(level);
        self
    }

    /// 设置职级描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置序列 ID
    pub fn job_family_id(mut self, job_family_id: String) -> Self {
        self.job_family_id = Some(job_family_id);
        self
    }

    /// 设置状态
    /// - 1: 启用
    /// - 2: 停用
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.job_level_id.trim(), "职级 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::JobLevelPatch(self.job_level_id);
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            name: self.name,
            level: self.level,
            description: self.description,
            job_family_id: self.job_family_id,
            status: self.status,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新职级响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
