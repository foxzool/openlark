//! 简历解析
//!
//! 智能解析简历文档，提取关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/resume_parse>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_RESUME_PARSE;

/// 简历解析请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeParseBody {
    /// 简历文件，支持 PDF / DOCX / PNG / JPG。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl ResumeParseBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 简历解析响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeParseResponse {
    /// 简历信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumes: Option<Vec<Resume>>,
}

impl openlark_core::api::ApiResponseTrait for ResumeParseResponse {}

/// 简历信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    /// 文件标识 ID，依据文件内容自动生成。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_md5: Option<String>,
    /// 文本内容，当接口返回成功时，该字段才存在。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 经过排序后的文本内容，当接口返回成功时，该字段才存在。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_content: Option<String>,
    /// 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 手机号码是否虚拟号码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_is_virtual: Option<bool>,
    /// 手机号码国家编码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 教育经历。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub educations: Option<Vec<ResumeEducation>>,
    /// 职业经历。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub careers: Option<Vec<ResumeCareer>>,
    /// 项目经历。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ResumeProject>>,
    /// 工作年限，为空表示工作年限未知，数字单位为年，整数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_year: Option<i32>,
    /// 生日，格式 YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// 性别：0 未知、1 男性、2 女性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 希望获得的职位列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willing_positions: Option<Vec<String>>,
    /// 当前工作地点（城市）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_location: Option<String>,
    /// 希望工作地点列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub willing_locations: Option<Vec<String>>,
    /// 家乡（城市）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
    /// 语言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<ResumeLanguage>>,
    /// 获奖。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<ResumeAward>>,
    /// 证书。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<ResumeCertificate>>,
    /// 竞赛。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitions: Option<Vec<ResumeCompetition>>,
    /// 自我评价。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_evaluation: Option<String>,
    /// 链接列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    /// 社交链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<String>>,
}

/// 教育经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeEducation {
    /// 学校名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD，跟 start_date 值一样。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD 或“至今”，当值为“至今”时 end_date 为空字符串。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 专业。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历——小学、初中、中职、高中、专科、本科、硕士、博士、其他。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 学历对应 ID：1 小学、2 初中、3 中职、4 高中、5 专科、6 本科、7 硕士、8 博士、9 其他。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<i32>,
}

/// 职业经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeCareer {
    /// 公司名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD，跟 start_date 值一样。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD 或“至今”，当值为“至今”时 end_date 为空字符串。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 职位。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作类型：1 实习、2 全职。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub work_type: Option<i32>,
    /// 工作类型——“实习”、“全职”。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_str: Option<String>,
    /// 工作描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
}

/// 项目经历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeProject {
    /// 项目名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 项目岗位。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 开始时间，格式：YYYY-MM-DD，跟 start_date 值一样。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 结束时间，格式：YYYY-MM-DD 或“至今”，当值为“至今”时 end_date 为空字符串。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 项目描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 语言
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeLanguage {
    /// 语言等级。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 语言描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 获奖
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeAward {
    /// 奖项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award: Option<String>,
    /// 获奖时间，格式 YYYY。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 证书
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeCertificate {
    /// 证书名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 竞赛
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeCompetition {
    /// 竞赛名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 简历解析请求
#[derive(Debug, Clone)]
pub struct ResumeParseRequest {
    config: Config,
}

impl ResumeParseRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: ResumeParseBody) -> SDKResult<ResumeParseResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: ResumeParseBody,
        option: RequestOption,
    ) -> SDKResult<ResumeParseResponse> {
        body.validate()?;

        let req: ApiRequest<ResumeParseResponse> = ApiRequest::post(DOCUMENT_AI_RESUME_PARSE)
            .body(serialize_params(&body, "简历解析")?)
            .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "简历解析").await
    }
}

/// 简历解析请求构建器
#[derive(Debug, Clone)]
pub struct ResumeParseRequestBuilder {
    request: ResumeParseRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl ResumeParseRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: ResumeParseRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置简历文件（支持 PDF / DOCX / PNG / JPG）。
    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// 设置 multipart 文件名。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> ResumeParseBody {
        ResumeParseBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ResumeParseResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ResumeParseResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行简历解析。
pub async fn resume_parse(
    config: &Config,
    body: ResumeParseBody,
) -> SDKResult<ResumeParseResponse> {
    resume_parse_with_options(config, body, RequestOption::default()).await
}

/// resume_parse_with_options。
pub async fn resume_parse_with_options(
    config: &Config,
    body: ResumeParseBody,
    option: RequestOption,
) -> SDKResult<ResumeParseResponse> {
    body.validate()?;

    let req: ApiRequest<ResumeParseResponse> = ApiRequest::post(DOCUMENT_AI_RESUME_PARSE)
        .body(serialize_params(&body, "简历解析")?)
        .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "简历解析").await
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
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = ResumeParseBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = ResumeParseBody {
            file: b"resume file".to_vec(),
            file_name: None,
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = ResumeParseRequestBuilder::new(config.clone())
            .file(b"resume file".to_vec())
            .file_name("resume.pdf")
            .body();
        assert_eq!(body.file, b"resume file".to_vec());
        assert_eq!(body.file_name, Some("resume.pdf".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = ResumeParseBody {
            file: b"resume file".to_vec(),
            file_name: Some("resume.pdf".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("resume.pdf"));
    }

    #[test]
    fn test_resume_parse_response_deserialization() {
        let payload = serde_json::json!({
            "code": 0,
            "msg": "success",
            "data": {
                "resumes": [
                    {
                        "file_md5": "825c59042dxxxxx3ff90b45xxxxx88",
                        "content": "XX负责行政人事管理和日常事务...",
                        "new_content": "XX负责行政人事管理和日常事务...",
                        "name": "张三",
                        "email": "zhangsan.1111@company.com",
                        "mobile": "13600000000",
                        "mobile_is_virtual": false,
                        "country_code": "86",
                        "educations": [
                            {
                                "school": "A大学",
                                "start_date": "2020-01-03",
                                "start_time": "2020-01-03",
                                "end_date": "2021-01-03",
                                "end_time": "至今",
                                "major": "XX工程",
                                "degree": "本科",
                                "qualification": 6
                            }
                        ],
                        "careers": [
                            {
                                "company": "XX公司",
                                "start_date": "2022-01-03",
                                "start_time": "2022-01-03",
                                "end_date": "2023-01-03",
                                "end_time": "2023-01-03",
                                "title": "XXX工程师",
                                "type": 2,
                                "type_str": "全职",
                                "job_description": "负责XXX开发..."
                            }
                        ],
                        "projects": [
                            {
                                "name": "XX项目",
                                "title": "客户端研发",
                                "start_date": "2023-01-03",
                                "start_time": "2023-01-03",
                                "end_date": "2023-01-04",
                                "end_time": "2023-01-04",
                                "description": "XXX项目是一个..."
                            }
                        ],
                        "work_year": 5,
                        "date_of_birth": "1995-01-01",
                        "gender": 1,
                        "willing_positions": ["XX岗位"],
                        "current_location": "上海",
                        "willing_locations": ["上海"],
                        "home_location": "上海",
                        "languages": [
                            { "level": 2, "description": "英语四级:600" }
                        ],
                        "awards": [
                            { "award": "XXX大赛奖项", "date": "2015", "description": "曾获XXX大赛奖项..." }
                        ],
                        "certificates": [
                            { "name": "XXX证书", "desc": "曾获得XXX证书..." }
                        ],
                        "competitions": [
                            { "name": "XXX竞赛", "desc": "曾参加XXX竞赛..." }
                        ],
                        "self_evaluation": "我是一个...",
                        "urls": ["https://github.com/"],
                        "social_links": ["https://github.com/"]
                    }
                ]
            }
        });
        // request_typed 会把 envelope 的 data 内容反序列化为 ResumeParseResponse，
        // 这里等价地只反序列化 data 部分。
        let data = payload.get("data").cloned().expect("payload 缺少 data");
        let response: ResumeParseResponse = serde_json::from_value(data).expect("反序列化失败");
        let resume = response
            .resumes
            .as_ref()
            .and_then(|resumes| resumes.first())
            .expect("resumes 为空");
        assert_eq!(resume.name.as_deref(), Some("张三"));
        assert_eq!(resume.mobile_is_virtual, Some(false));
        assert_eq!(
            resume
                .educations
                .as_ref()
                .and_then(|educations| educations.first())
                .and_then(|education| education.qualification),
            Some(6)
        );
        assert_eq!(
            resume
                .careers
                .as_ref()
                .and_then(|careers| careers.first())
                .and_then(|career| career.work_type),
            Some(2)
        );
    }
}
