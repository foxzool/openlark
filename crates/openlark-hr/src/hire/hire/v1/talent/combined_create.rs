//! 综合创建候选人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/combined_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{
    CombinedCreateRequestBody, CombinedCreateResponse, TalentEducation, TalentWorkExperience,
};

/// 综合创建候选人请求
#[derive(Debug, Clone)]
pub struct CombinedCreateRequest {
    /// 配置信息
    config: Config,
    /// 候选人姓名（必填）
    name: String,
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

impl CombinedCreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            email: None,
            phone: None,
            resume: None,
            education_list: None,
            work_experience_list: None,
        }
    }

    /// 设置候选人姓名（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
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
    pub async fn execute(self) -> SDKResult<CombinedCreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedCreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        // 1. 验证必填字段
        validate_required!(self.name.trim(), "候选人姓名不能为空");

        // 验证至少有一个联系方式
        let has_contact = self.email.as_ref().map(|e| !e.is_empty()).unwrap_or(false)
            || self.phone.as_ref().map(|p| !p.is_empty()).unwrap_or(false);
        if !has_contact {
            return Err(openlark_core::error::validation_error(
                "缺少联系方式",
                "创建候选人时至少需要提供邮箱或手机号",
            ));
        }

        // 2. 构建端点
        let api_endpoint = HireApiV1::TalentCombinedCreate;
        let request = ApiRequest::<CombinedCreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CombinedCreateRequestBody {
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
                "综合创建候选人响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CombinedCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_talent_combined_create_request_builder() {
        let request = CombinedCreateRequest::new(create_test_config())
            .name("张三".to_string())
            .email("zhangsan@example.com".to_string())
            .resume("<p>简历内容</p>".to_string());

        assert_eq!(request.name, "张三");
        assert_eq!(request.email.as_deref(), Some("zhangsan@example.com"));
        assert_eq!(request.resume.as_deref(), Some("<p>简历内容</p>"));
    }

    #[test]
    fn test_talent_combined_create_request_body_serialize() {
        let request = CombinedCreateRequest::new(create_test_config())
            .name("李四".to_string())
            .phone("13800000000".to_string());

        let body = CombinedCreateRequestBody {
            name: request.name,
            email: request.email,
            phone: request.phone,
            resume: request.resume,
            education_list: request.education_list,
            work_experience_list: request.work_experience_list,
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["name"], json!("李四"));
        assert_eq!(value["phone"], json!("13800000000"));
        assert!(value.get("email").is_none());
    }

    #[test]
    fn test_talent_combined_create_response_deserialize() {
        let value = json!({"talent_id": "talent_123"});
        let response: CombinedCreateResponse =
            serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.talent_id, "talent_123");
    }

    #[test]
    fn test_talent_combined_create_validation() {
        let request = CombinedCreateRequest::new(create_test_config()).name("   ".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.name.trim(), "候选人姓名不能为空");
            let has_contact = request
                .email
                .as_ref()
                .map(|email| !email.is_empty())
                .unwrap_or(false)
                || request
                    .phone
                    .as_ref()
                    .map(|phone| !phone.is_empty())
                    .unwrap_or(false);
            if !has_contact {
                return Err(openlark_core::error::validation_error(
                    "缺少联系方式",
                    "创建候选人时至少需要提供邮箱或手机号",
                ));
            }
            Ok(())
        })();

        assert!(result.is_err());
    }
}
