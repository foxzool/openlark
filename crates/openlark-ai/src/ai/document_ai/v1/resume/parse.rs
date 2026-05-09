//! 简历解析
//!
//! 智能解析简历文档，提取关键信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/resume_parse

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_RESUME_PARSE;

/// 简历解析请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeParseBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl ResumeParseBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 简历解析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeParseResponse {
    /// 简历解析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResumeParseResult>,
}

impl openlark_core::api::ApiResponseTrait for ResumeParseResponse {}

/// 简历解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeParseResult {
    /// 解析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingResult {
    /// 简历 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_id: Option<String>,
    /// 简历来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_source: Option<String>,
    /// 候选人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_name: Option<String>,
    /// 候选人性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_gender: Option<String>,
    /// 候选人年龄
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_age: Option<String>,
    /// 候选人职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
    /// 工作年限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_years: Option<String>,
    /// 最高学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_education: Option<String>,
    /// 当前所在公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_company: Option<String>,
    /// 当前职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_position: Option<String>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experiences: Option<Vec<WorkExperience>>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_experiences: Option<Vec<EducationExperience>>,
    /// 项目经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_experiences: Option<Vec<ProjectExperience>>,
    /// 证书
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<String>>,
    /// 技能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    /// 简历原文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
}

/// 工作经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkExperience {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 教育经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationExperience {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 学位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 项目经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectExperience {
    /// 项目名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// 角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 项目描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 简历解析请求
#[derive(Debug, Clone)]
pub struct ResumeParseRequest {
    config: Config,
}

impl ResumeParseRequest {
    /// 创建新的简历解析请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行简历解析请求
    pub async fn execute(self, body: ResumeParseBody) -> SDKResult<ResumeParseResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行简历解析请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: ResumeParseBody,
        option: RequestOption,
    ) -> SDKResult<ResumeParseResponse> {
        body.validate()?;

        let req: ApiRequest<ResumeParseResponse> =
            ApiRequest::post(DOCUMENT_AI_RESUME_PARSE).body(serialize_params(&body, "简历解析")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "简历解析")
    }
}

/// 简历解析请求构建器
#[derive(Debug, Clone)]
pub struct ResumeParseRequestBuilder {
    request: ResumeParseRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl ResumeParseRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: ResumeParseRequest::new(config),
            file_token: None,
            is_async: None,
        }
    }

    /// 设置文件 token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置是否异步
    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> ResumeParseBody {
        ResumeParseBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ResumeParseResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ResumeParseResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行简历解析
///
/// docPath: https://open.feishu.cn/document/document_ai-v1/resume_parse
pub async fn resume_parse(
    config: &Config,
    body: ResumeParseBody,
) -> SDKResult<ResumeParseResponse> {
    resume_parse_with_options(config, body, RequestOption::default()).await
}

/// 执行简历解析（支持自定义选项）
pub async fn resume_parse_with_options(
    config: &Config,
    body: ResumeParseBody,
    option: RequestOption,
) -> SDKResult<ResumeParseResponse> {
    body.validate()?;

    let req: ApiRequest<ResumeParseResponse> =
        ApiRequest::post(DOCUMENT_AI_RESUME_PARSE).body(serialize_params(&body, "简历解析")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "简历解析")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ResumeParseRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_builder_file_token() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ResumeParseRequestBuilder::new(config.clone()).file_token("file_token_123");

        assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_builder_is_async() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ResumeParseRequestBuilder::new(config.clone()).is_async(true);

        assert_eq!(builder.is_async, Some(true));
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = ResumeParseBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = ResumeParseBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_from_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = ResumeParseRequestBuilder::new(config.clone())
            .file_token("token_123")
            .is_async(true)
            .body();

        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.is_async, Some(true));
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = ResumeParseBody {
            file_token: "   ".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization() {
        let body = ResumeParseBody {
            file_token: "token_123".to_string(),
            is_async: Some(true),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("file_token"));
        assert!(json_str.contains("token_123"));
    }

    #[test]
    fn test_parsing_result_struct() {
        let parsing_result = ParsingResult {
            resume_id: Some("resume_001".to_string()),
            resume_source: Some("upload".to_string()),
            candidate_name: Some("李四".to_string()),
            candidate_gender: Some("男".to_string()),
            candidate_age: Some("28".to_string()),
            target_position: Some("软件工程师".to_string()),
            work_years: Some("5".to_string()),
            highest_education: Some("本科".to_string()),
            current_company: Some("ABC科技".to_string()),
            current_position: Some("高级工程师".to_string()),
            work_experiences: None,
            education_experiences: None,
            project_experiences: None,
            certificates: None,
            skills: None,
            raw_text: None,
        };
        assert_eq!(parsing_result.candidate_name, Some("李四".to_string()));
    }

    #[test]
    fn test_work_experience_struct() {
        let work_exp = WorkExperience {
            company: Some("ABC公司".to_string()),
            position: Some("工程师".to_string()),
            start_time: Some("2020-01".to_string()),
            end_time: Some("2023-12".to_string()),
            description: Some("负责后端开发".to_string()),
        };
        assert_eq!(work_exp.company, Some("ABC公司".to_string()));
        assert_eq!(work_exp.position, Some("工程师".to_string()));
    }

    #[test]
    fn test_work_experience_serialization() {
        let work_exp = WorkExperience {
            company: Some("ABC公司".to_string()),
            position: Some("工程师".to_string()),
            start_time: Some("2020-01".to_string()),
            end_time: Some("2023-12".to_string()),
            description: Some("负责后端开发".to_string()),
        };
        let json_str = serde_json::to_string(&work_exp).expect("序列化失败");
        assert!(json_str.contains("ABC公司"));
        assert!(json_str.contains("工程师"));
    }

    #[test]
    fn test_education_experience_struct() {
        let edu_exp = EducationExperience {
            school: Some("清华大学".to_string()),
            degree: Some("硕士".to_string()),
            major: Some("计算机科学".to_string()),
            start_time: Some("2015-09".to_string()),
            end_time: Some("2018-06".to_string()),
        };
        assert_eq!(edu_exp.school, Some("清华大学".to_string()));
        assert_eq!(edu_exp.degree, Some("硕士".to_string()));
    }

    #[test]
    fn test_project_experience_struct() {
        let proj_exp = ProjectExperience {
            project_name: Some("智能客服系统".to_string()),
            role: Some("技术负责人".to_string()),
            start_time: Some("2021-03".to_string()),
            end_time: Some("2022-06".to_string()),
            description: Some("负责系统架构设计".to_string()),
        };
        assert_eq!(proj_exp.project_name, Some("智能客服系统".to_string()));
        assert_eq!(proj_exp.role, Some("技术负责人".to_string()));
    }

    #[test]
    fn test_resume_parsing_result_serialization() {
        let parsing_result = ParsingResult {
            resume_id: Some("resume_001".to_string()),
            resume_source: Some("upload".to_string()),
            candidate_name: Some("李四".to_string()),
            candidate_gender: Some("男".to_string()),
            candidate_age: Some("28".to_string()),
            target_position: Some("软件工程师".to_string()),
            work_years: Some("5".to_string()),
            highest_education: Some("本科".to_string()),
            current_company: Some("ABC科技".to_string()),
            current_position: Some("高级工程师".to_string()),
            work_experiences: None,
            education_experiences: None,
            project_experiences: None,
            certificates: Some(vec!["PMP".to_string(), "AWS".to_string()]),
            skills: Some(vec!["Rust".to_string(), "Python".to_string()]),
            raw_text: Some("简历原文".to_string()),
        };
        let json_str = serde_json::to_string(&parsing_result).expect("序列化失败");
        assert!(json_str.contains("李四"));
        assert!(json_str.contains("软件工程师"));
        assert!(json_str.contains("Rust"));
    }

    #[test]
    fn test_response_struct() {
        let response = ResumeParseResponse { data: None };
        assert!(response.data.is_none());

        let result = ResumeParseResult {
            parsing_result: Some(ParsingResult {
                resume_id: Some("resume_002".to_string()),
                resume_source: None,
                candidate_name: Some("王五".to_string()),
                candidate_gender: None,
                candidate_age: None,
                target_position: None,
                work_years: None,
                highest_education: None,
                current_company: None,
                current_position: None,
                work_experiences: None,
                education_experiences: None,
                project_experiences: None,
                certificates: None,
                skills: None,
                raw_text: None,
            }),
        };
        let response_with_data = ResumeParseResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        let candidate_name = response_with_data
            .data
            .as_ref()
            .and_then(|data| data.parsing_result.as_ref())
            .and_then(|result| result.candidate_name.as_deref());
        assert_eq!(candidate_name, Some("王五"));
    }
}
