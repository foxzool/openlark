//! 综合更新候选人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/combined_update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{
    CombinedUpdateRequestBody, CombinedUpdateResponse, TalentEducation, TalentWorkExperience,
};

/// 综合更新候选人请求
#[derive(Debug, Clone)]
pub struct CombinedUpdateRequest {
    /// 配置信息
    config: Config,
    /// 候选人 ID（必填）
    talent_id: String,
    /// 候选人姓名
    name: Option<String>,
    /// 邮箱
    email: Option<String>,
    /// 手机号
    phone: Option<String>,
    /// 简历内容（HTML格式）
    resume: Option<String>,
    /// 教育经历
    education_list: Option<Vec<TalentEducation>>,
    /// 工作经历
    work_experience_list: Option<Vec<TalentWorkExperience>>,
}

impl CombinedUpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
            name: None,
            email: None,
            phone: None,
            resume: None,
            education_list: None,
            work_experience_list: None,
        }
    }

    /// 设置候选人 ID（必填）
    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 设置候选人姓名
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置手机号
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// 设置简历内容（HTML格式）
    pub fn resume(mut self, resume: String) -> Self {
        self.resume = Some(resume);
        self
    }

    /// 设置教育经历
    pub fn education_list(mut self, education_list: Vec<TalentEducation>) -> Self {
        self.education_list = Some(education_list);
        self
    }

    /// 设置工作经历
    pub fn work_experience_list(mut self, work_experience_list: Vec<TalentWorkExperience>) -> Self {
        self.work_experience_list = Some(work_experience_list);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedUpdateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 1. 验证必填字段
        validate_required!(self.talent_id.trim(), "候选人 ID 不能为空");

        // 验证至少有一个更新字段
        let has_update = self.name.is_some()
            || self.email.is_some()
            || self.phone.is_some()
            || self.resume.is_some()
            || self.education_list.is_some()
            || self.work_experience_list.is_some();
        if !has_update {
            return Err(openlark_core::error::validation_error(
                "缺少更新字段",
                "更新候选人时至少需要提供一个更新字段",
            ));
        }

        // 2. 构建端点
        let api_endpoint = HireApiV1::TalentCombinedUpdate;
        let request = ApiRequest::<CombinedUpdateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CombinedUpdateRequestBody {
            talent_id: self.talent_id,
            name: self.name,
            email: self.email,
            phone: self.phone,
            resume: self.resume,
            education_list: self.education_list,
            work_experience_list: self.work_experience_list,
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
                "综合更新候选人响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CombinedUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_combined_update_request_builder_new() {
        let request = CombinedUpdateRequest::new(TestConfigBuilder::new().build())
            .talent_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_combined_update_request_validation_fails_on_default_request() {
        let request = CombinedUpdateRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
