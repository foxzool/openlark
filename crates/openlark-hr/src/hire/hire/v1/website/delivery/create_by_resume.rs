//! 新建招聘官网投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/website.delivery/create_by_resume>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::WebsiteDeliveryDto;

/// 自定义字段（官方 `website_delivery_customized_data`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryCustomizedData {
    /// 自定义字段 ID，可通过获取人才字段接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 自定义字段值，请参考官方文档「自定义字段数据格式说明」
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义模块（官方 `website_delivery_customized_data_parent`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryCustomizedDataParent {
    /// 自定义模块 ID，可通过获取人才字段接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 证件信息（官方 `website_delivery_identification`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryIdentification {
    /// 身份证件号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
    /// 身份证件类型：1 中国-居民身份证 / 2 护照 / 3 港澳居民居住证 / 4 台湾居民来往大陆通行证 / 5 其他 / 6 港澳居民来往内地通行证 / 9 台湾居民居住证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
}

/// 基本信息（官方 `website_delivery_basic_info`；`name` 必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryBasicInfo {
    /// 姓名（必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 国家码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 起始工作时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_work_time: Option<i64>,
    /// 家庭住址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_home_address: Option<String>,
    /// 家乡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hometown_city_code: Option<String>,
    /// 手机国家代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_country_code: Option<String>,
    /// 身份证件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<WebsiteDeliveryIdentification>,
    /// 婚姻状况：1 已婚 / 2 未婚
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i32>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 所在城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_city_code: Option<String>,
    /// 工作年限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_years: Option<i32>,
    /// 性别：1 男 / 2 女 / 3 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 出生日期，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    /// 意向城市列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_city_code_list: Option<Vec<String>>,
    /// 简历来源，可通过获取简历来源列表接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_source_id: Option<String>,
    /// 年龄
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 实习经历（官方 `website_delivery_internship`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryInternship {
    /// 实习经历描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 结束时间，毫秒时间戳，「至今」可传 -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 教育经历（官方 `website_delivery_education`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryEducation {
    /// 学历类型：1 非中国大陆 / 2 统招全日制 / 3 非全日制 / 4 自考 / 5 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_type: Option<i32>,
    /// 结束时间，毫秒时间戳，「至今」可传 -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 结束时间（新），毫秒时间戳，无「至今」传值（建议使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_v2: Option<i64>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    /// 学校
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 专业排名：5 前 5% / 10 前 10% / 20 前 20% / 30 前 30% / 50 前 50% / 51 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub academic_ranking: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 学位：1 小学 / 2 初中 / 3 专职 / 4 高中 / 5 大专 / 6 本科 / 7 硕士 / 8 博士 / 9 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<i32>,
}

/// 自我评价（官方 `website_delivery_self_evaluation`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliverySelfEvaluation {
    /// 评价内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 工作经历（官方 `website_delivery_career`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryCareer {
    /// 工作经历描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 结束时间，毫秒时间戳，「至今」可传 -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 社交账号（官方 `website_delivery_sns`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliverySns {
    /// 社交账号类型：1 领英 / 2 脉脉 / 3 微信 / 4 微博 / 5 Github / 6 知乎 …（见官方枚举）
    #[serde(rename = "sns-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_type: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 社交账号链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// 作品附件（官方 `website_delivery_works_attachment`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryWorksAttachment {
    /// 文件 ID，需通过创建附件生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 作品（官方 `website_delivery_works`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryWorks {
    /// 作品描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 作品链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<WebsiteDeliveryWorksAttachment>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
}

/// 获奖记录（官方 `website_delivery_award`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryAward {
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 获奖描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 奖项名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 获奖时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_time: Option<i64>,
}

/// 项目经历（官方 `website_delivery_project`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryProject {
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 项目描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 结束时间，毫秒时间戳，「至今」可传 -1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 项目链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 项目名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 项目角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

/// 语言能力（官方 `website_delivery_language`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryLanguage {
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedData>>,
    /// 语言（见官方枚举）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    /// 熟悉程度：1 入门 / 2 日常会话 / 3 商务会话 / 4 无障碍沟通 / 5 母语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<i32>,
}

/// 人才信息（官方 `website_delivery_resume`；`basic_info` 必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryResume {
    /// 实习经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internship_list: Option<Vec<WebsiteDeliveryInternship>>,
    /// 基本信息（必填，其内 `name` 必填）
    pub basic_info: WebsiteDeliveryBasicInfo,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<WebsiteDeliveryEducation>>,
    /// 自我评价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_evaluation: Option<WebsiteDeliverySelfEvaluation>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_list: Option<Vec<WebsiteDeliveryCareer>>,
    /// 自定义模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<WebsiteDeliveryCustomizedDataParent>>,
    /// 简历附件 ID，需通过创建附件生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_attachment_id: Option<String>,
    /// 社交账号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_list: Option<Vec<WebsiteDeliverySns>>,
    /// 作品
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<WebsiteDeliveryWorks>>,
    /// 获奖记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_list: Option<Vec<WebsiteDeliveryAward>>,
    /// 项目经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_list: Option<Vec<WebsiteDeliveryProject>>,
    /// 语言能力
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_list: Option<Vec<WebsiteDeliveryLanguage>>,
}

/// 新建招聘官网投递请求体（官方必填 `job_post_id`/`resume`/`user_id`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateByResumeBody {
    /// 职位广告 ID（必填），可通过搜索/获取招聘官网下的职位广告列表接口获取
    pub job_post_id: String,
    /// 人才信息（必填，其内 `basic_info.name` 必填）
    pub resume: WebsiteDeliveryResume,
    /// 官网用户 ID（必填），需通过新建招聘官网用户生成
    pub user_id: String,
    /// 意向投递城市列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_city_code_list: Option<Vec<String>>,
    /// 官网推广渠道 ID，可通过获取招聘官网推广渠道列表接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

/// 新建招聘官网投递请求
#[derive(Debug, Clone)]
pub struct CreateByResumeRequest {
    /// 配置信息
    config: Config,
    website_id: Option<String>,
    body: CreateByResumeBody,
}

impl CreateByResumeRequest {
    /// 创建请求（`job_post_id`/`resume`/`user_id` 为官方必填字段）
    pub fn new(
        config: Config,
        job_post_id: impl Into<String>,
        resume: WebsiteDeliveryResume,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            website_id: None,
            body: CreateByResumeBody {
                job_post_id: job_post_id.into(),
                resume,
                user_id: user_id.into(),
                application_preferred_city_code_list: None,
                channel_id: None,
            },
        }
    }

    /// 设置 `website_id`。
    pub fn website_id(mut self, website_id: impl Into<String>) -> Self {
        self.website_id = Some(website_id.into());
        self
    }

    /// 设置意向投递城市列表。
    pub fn application_preferred_city_code_list(mut self, value: Vec<String>) -> Self {
        self.body.application_preferred_city_code_list = Some(value);
        self
    }

    /// 设置官网推广渠道 ID。
    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.body.channel_id = Some(channel_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateByResumeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateByResumeResponse> {
        let website_id = self.website_id.unwrap_or_default();
        validate_required!(website_id.trim(), "website_id 不能为空");
        validate_required!(self.body.job_post_id.trim(), "job_post_id 不能为空");
        validate_required!(self.body.user_id.trim(), "user_id 不能为空");
        let name = self.body.resume.basic_info.name.clone().unwrap_or_default();
        validate_required!(name.trim(), "resume.basic_info.name 不能为空");

        let request = ApiRequest::<CreateByResumeResponse>::post(format!(
            "/open-apis/hire/v1/websites/{website_id}/deliveries/create_by_resume"
        ))
        .body(serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "新建招聘官网投递响应数据为空",
        )
        .await
    }
}

/// 新建招聘官网投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateByResumeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递信息。
    pub delivery: Option<WebsiteDeliveryDto>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateByResumeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/websites/test001/deliveries/create_by_resume，
    /// 请求体按官方形状（job_post_id/resume/user_id 必填），响应按官方 `data.delivery` 形状解析。
    #[tokio::test]
    async fn test_create_by_resume_sends_official_body_shape() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/websites/test001/deliveries/create_by_resume",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "ok",
                "data": {
                    "delivery": {
                        "application_id": "6960663240925956657",
                        "id": "6960663240925956655",
                        "job_id": "6960663240925956659",
                        "job_post_id": "6960663240925956658",
                        "portal_resume_id": "6960663240925956660",
                        "user_id": "6960663240925956656",
                        "talent_id": "7095600054216542508"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resume = WebsiteDeliveryResume {
            basic_info: WebsiteDeliveryBasicInfo {
                name: Some("张三".to_string()),
                email: Some("foo@bytedance.com".to_string()),
                mobile: Some("182900291190".to_string()),
                ..Default::default()
            },
            education_list: Some(vec![WebsiteDeliveryEducation {
                school: Some("香港中文大学".to_string()),
                field_of_study: Some("汉语言文学".to_string()),
                degree: Some(6),
                start_time: Some(1609430400000),
                end_time: Some(1618500278663),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let resp = CreateByResumeRequest::new(
            config,
            "6960663240925956636".to_string(),
            resume,
            "6960663240925956634".to_string(),
        )
        .website_id("test001".to_string())
        .execute()
        .await
        .expect("请求应成功");

        let delivery = resp.delivery.expect("应解析出 delivery");
        assert_eq!(
            delivery.application_id.as_deref(),
            Some("6960663240925956657")
        );
        assert_eq!(delivery.id.as_deref(), Some("6960663240925956655"));
        assert_eq!(delivery.job_id.as_deref(), Some("6960663240925956659"));
        assert_eq!(delivery.job_post_id.as_deref(), Some("6960663240925956658"));
        assert_eq!(
            delivery.portal_resume_id.as_deref(),
            Some("6960663240925956660")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：job_post_id/resume/user_id 必填，resume.basic_info.name 必填
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({
                "job_post_id": "6960663240925956636",
                "user_id": "6960663240925956634",
                "resume": {
                    "basic_info": {
                        "name": "张三",
                        "email": "foo@bytedance.com",
                        "mobile": "182900291190"
                    },
                    "education_list": [
                        {
                            "school": "香港中文大学",
                            "field_of_study": "汉语言文学",
                            "degree": 6,
                            "start_time": 1609430400000i64,
                            "end_time": 1618500278663i64
                        }
                    ]
                }
            })
        );
    }

    /// 缺必填字段（resume.basic_info.name）时快速失败，不发起请求。
    #[tokio::test]
    async fn test_create_by_resume_rejects_missing_basic_info_name() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = CreateByResumeRequest::new(
            config,
            "6960663240925956636".to_string(),
            WebsiteDeliveryResume::default(),
            "6960663240925956634".to_string(),
        )
        .website_id("test001".to_string())
        .execute()
        .await
        .expect_err("缺 resume.basic_info.name 应被拒绝");
        assert!(err.to_string().contains("name"), "实际: {err}");
    }
}
