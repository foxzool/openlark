//! Hire v1 talent 资源相关模型
//!
//! 包含获取候选人列表、获取候选人信息、综合创建/更新候选人、批量获取候选人ID等API的请求和响应结构体

use serde::{Deserialize, Serialize};

// 候选人基础数据结构

/// 候选人信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Talent {
    /// 候选人 ID
    pub talent_id: String,
    /// 候选人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 简历内容（HTML格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 候选人状态
    /// - 1: 新候选人
    /// - 2: 在流程中
    /// - 3: 已归档
    /// - 4: 已入职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

/// 候选人联系方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentContact {
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// 候选人教育经历
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentEducation {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 候选人工作经历
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentWorkExperience {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// 获取候选人列表相关模型

/// 获取候选人列表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 查询关键词（姓名、邮箱、手机号）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 候选人状态列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<i32>>,
}

/// 获取候选人列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 候选人列表
    pub talent_list: Vec<Talent>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 获取候选人信息相关模型

/// 获取候选人信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 是否需要详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_detail: Option<bool>,
}

/// 获取候选人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 候选人信息
    pub talent: Talent,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentEducation>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<TalentWorkExperience>>,
}

// 综合创建候选人相关模型

/// 人才自定义字段值（官方 `talent_customized_data_object_value`）
///
/// 注意：`children` 字段无效，请参考请求体示例在当前层级传入 `object_id` 和 `value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedDataObjectValue {
    /// 自定义字段 ID，可通过获取人才字段接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 子字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TalentCustomizedDataObjectValueChild>>,
    /// 自定义字段值，格式见官方文档「自定义字段数据格式说明」
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 人才自定义字段子项（官方 `talent_customized_data_object_value_child`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedDataObjectValueChild {
    /// 自定义字段 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 自定义字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 人才证件信息（官方 `talent_identification_info`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentIdentificationInfo {
    /// 证件类型：1 中国-居民身份证 / 2 护照 / 3 港澳居民居住证 / 4 台湾居民来往大陆通行证 / 5 其他 / 6 港澳居民来往内地通行证 / 9 台湾居民居住证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
    /// 证件号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
}

/// 人才基本信息（官方 `talent_combined_basic_info`；创建时 `name` 必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedBasicInfo {
    /// 名字（创建时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 手机
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 手机国家代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_country_code: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 证件信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<TalentIdentificationInfo>,
    /// 开始工作时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_work_time: Option<String>,
    /// 出生日期时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// 性别：1 男 / 2 女 / 3 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 国籍编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 所在地点编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_city_code: Option<String>,
    /// 家乡编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hometown_city_code: Option<String>,
    /// 自定义字段列表（`children` 字段无效，请在当前层级传入 `object_id` 和 `value`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才教育经历（官方 `talent_combined_education_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedEducationInfo {
    /// 学校
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 学历：1 小学 / 2 初中 / 3 专职 / 4 高中 / 5 大专 / 6 本科 / 7 硕士 / 8 博士 / 9 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<i32>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,
    /// 教育经历开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 教育经历结束时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 学历类型：1 海外及港台 / 2 统招全日制 / 3 非全日制 / 4 自考 / 5 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_type: Option<i32>,
    /// 成绩排名：5 前 5% / 10 前 10% / 20 前 20% / 30 前 30% / 50 前 50% / -1 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub academic_ranking: Option<i32>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才工作经历（官方 `talent_combined_career_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedCareerInfo {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作经历描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 工作经历开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 工作经历结束时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 工作经历类型：1 实习经历 / 2 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_type: Option<i32>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才项目经历（官方 `talent_combined_project_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedProjectInfo {
    /// 项目名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 项目角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 项目链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 项目描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 项目开始时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 项目结束时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才作品（官方 `talent_combined_work_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedWorkInfo {
    /// 作品链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 作品描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 作品附件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才获奖记录（官方 `talent_combined_award_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedAwardInfo {
    /// 获奖名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 获奖时间，毫秒时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_time: Option<String>,
    /// 获奖描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才语言能力（官方 `talent_combined_language_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedLanguageInfo {
    /// 语言类型：1 英语 / 2 法语 / 3 日语 …（见官方枚举）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    /// 语言精通程度：1 入门 / 2 日常会话 / 3 商务会话 / 4 无障碍沟通 / 5 母语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<i32>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才社交账号（官方 `talent_combined_sns_info`；更新时全量覆盖，最多 100 条）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCombinedSnsInfo {
    /// 社交平台：1 领英 / 2 脉脉 / 3 微信 / 4 微博 / 5 Github / 6 知乎 …（见官方枚举）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_type: Option<i32>,
    /// URL/ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 人才自我评价（官方 `talent_self_evaluation`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentSelfEvaluation {
    /// 自我评价内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

/// 综合创建候选人请求体（官方必填 `basic_info`，其内 `name` 必填）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedCreateRequestBody {
    /// 简历来源 ID（已弃用，请使用 `resume_source_id`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_source_id: Option<String>,
    /// 简历来源 ID（`init_source_id` 已弃用，建议使用本字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_source_id: Option<String>,
    /// 文件夹 ID 列表（最多 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id_list: Option<Vec<String>>,
    /// 创建人 ID，和入参 `user_id_type` 类型保持一致；创建人类型为「系统」时无需填写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 创建人类型：1 员工 / 3 系统
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_account_type: Option<i32>,
    /// 简历附件 ID（仅增加附件简历，不解析内容）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_attachment_id: Option<String>,
    /// 基本信息（必填，`name` 必填）
    pub basic_info: TalentCombinedBasicInfo,
    /// 教育经历（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentCombinedEducationInfo>>,
    /// 工作经历（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_list: Option<Vec<TalentCombinedCareerInfo>>,
    /// 项目经历（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_list: Option<Vec<TalentCombinedProjectInfo>>,
    /// 作品（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<TalentCombinedWorkInfo>>,
    /// 获奖（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_list: Option<Vec<TalentCombinedAwardInfo>>,
    /// 语言能力（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_list: Option<Vec<TalentCombinedLanguageInfo>>,
    /// 社交账号（最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_list: Option<Vec<TalentCombinedSnsInfo>>,
    /// 意向地点（最多 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_city_code_list: Option<Vec<String>>,
    /// 自我评价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_evaluation: Option<TalentSelfEvaluation>,
    /// 自定义模块列表（`children` 为二维数组；更新时全量覆盖）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

impl CombinedCreateRequestBody {
    /// 创建请求体（`basic_info` 必填，其内 `name` 必填）。
    pub fn new(basic_info: TalentCombinedBasicInfo) -> Self {
        Self {
            init_source_id: None,
            resume_source_id: None,
            folder_id_list: None,
            creator_id: None,
            creator_account_type: None,
            resume_attachment_id: None,
            basic_info,
            education_list: None,
            career_list: None,
            project_list: None,
            works_list: None,
            award_list: None,
            language_list: None,
            sns_list: None,
            preferred_city_code_list: None,
            self_evaluation: None,
            customized_data: None,
        }
    }

    /// 设置简历来源 ID（已弃用，请使用 `resume_source_id`）。
    pub fn init_source_id(mut self, value: impl Into<String>) -> Self {
        self.init_source_id = Some(value.into());
        self
    }

    /// 设置简历来源 ID。
    pub fn resume_source_id(mut self, value: impl Into<String>) -> Self {
        self.resume_source_id = Some(value.into());
        self
    }

    /// 设置文件夹 ID 列表。
    pub fn folder_id_list(mut self, value: Vec<String>) -> Self {
        self.folder_id_list = Some(value);
        self
    }

    /// 设置创建人 ID。
    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    /// 设置创建人类型。
    pub fn creator_account_type(mut self, value: i32) -> Self {
        self.creator_account_type = Some(value);
        self
    }

    /// 设置简历附件 ID。
    pub fn resume_attachment_id(mut self, value: impl Into<String>) -> Self {
        self.resume_attachment_id = Some(value.into());
        self
    }

    /// 设置教育经历列表。
    pub fn education_list(mut self, value: Vec<TalentCombinedEducationInfo>) -> Self {
        self.education_list = Some(value);
        self
    }

    /// 设置工作经历列表。
    pub fn career_list(mut self, value: Vec<TalentCombinedCareerInfo>) -> Self {
        self.career_list = Some(value);
        self
    }

    /// 设置项目经历列表。
    pub fn project_list(mut self, value: Vec<TalentCombinedProjectInfo>) -> Self {
        self.project_list = Some(value);
        self
    }

    /// 设置作品列表。
    pub fn works_list(mut self, value: Vec<TalentCombinedWorkInfo>) -> Self {
        self.works_list = Some(value);
        self
    }

    /// 设置获奖列表。
    pub fn award_list(mut self, value: Vec<TalentCombinedAwardInfo>) -> Self {
        self.award_list = Some(value);
        self
    }

    /// 设置语言能力列表。
    pub fn language_list(mut self, value: Vec<TalentCombinedLanguageInfo>) -> Self {
        self.language_list = Some(value);
        self
    }

    /// 设置社交账号列表。
    pub fn sns_list(mut self, value: Vec<TalentCombinedSnsInfo>) -> Self {
        self.sns_list = Some(value);
        self
    }

    /// 设置意向地点列表。
    pub fn preferred_city_code_list(mut self, value: Vec<String>) -> Self {
        self.preferred_city_code_list = Some(value);
        self
    }

    /// 设置自我评价。
    pub fn self_evaluation(mut self, value: TalentSelfEvaluation) -> Self {
        self.self_evaluation = Some(value);
        self
    }

    /// 设置自定义模块列表。
    pub fn customized_data(mut self, value: Vec<TalentCustomizedDataObjectValue>) -> Self {
        self.customized_data = Some(value);
        self
    }
}

/// 综合创建候选人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedCreateResponse {
    /// 候选人 ID
    pub talent_id: String,
}

// 综合更新候选人相关模型

/// 综合更新候选人请求体（官方必填 `talent_id` 与 `basic_info`）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedUpdateRequestBody {
    /// 人才 ID（必填），可通过获取人才列表接口获取
    pub talent_id: String,
    /// 简历来源 ID（已弃用，请使用 `resume_source_id`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_source_id: Option<String>,
    /// 简历来源 ID（`init_source_id` 已弃用，建议使用本字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_source_id: Option<String>,
    /// 文件夹 ID 列表（最多 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id_list: Option<Vec<String>>,
    /// 更新人 ID，与入参 `user_id_type` 类型一致
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 更新人类型：1 员工 / 3 系统
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account_type: Option<i32>,
    /// 简历附件 ID（仅增加附件简历，不解析内容）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_attachment_id: Option<String>,
    /// 基本信息（必填）
    pub basic_info: TalentCombinedBasicInfo,
    /// 教育经历（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<TalentCombinedEducationInfo>>,
    /// 工作经历（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_list: Option<Vec<TalentCombinedCareerInfo>>,
    /// 项目经历（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_list: Option<Vec<TalentCombinedProjectInfo>>,
    /// 作品（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<TalentCombinedWorkInfo>>,
    /// 获奖（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_list: Option<Vec<TalentCombinedAwardInfo>>,
    /// 语言能力（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_list: Option<Vec<TalentCombinedLanguageInfo>>,
    /// 社交账号（更新时全量覆盖，最多 100 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_list: Option<Vec<TalentCombinedSnsInfo>>,
    /// 意向地点（最多 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_city_code_list: Option<Vec<String>>,
    /// 自我评价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_evaluation: Option<TalentSelfEvaluation>,
    /// 自定义模块列表（`children` 为二维数组；更新时全量覆盖）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data: Option<Vec<TalentCustomizedDataObjectValue>>,
}

impl CombinedUpdateRequestBody {
    /// 创建请求体（`talent_id` 与 `basic_info` 必填）。
    pub fn new(talent_id: impl Into<String>, basic_info: TalentCombinedBasicInfo) -> Self {
        Self {
            talent_id: talent_id.into(),
            init_source_id: None,
            resume_source_id: None,
            folder_id_list: None,
            operator_id: None,
            operator_account_type: None,
            resume_attachment_id: None,
            basic_info,
            education_list: None,
            career_list: None,
            project_list: None,
            works_list: None,
            award_list: None,
            language_list: None,
            sns_list: None,
            preferred_city_code_list: None,
            self_evaluation: None,
            customized_data: None,
        }
    }

    /// 设置简历来源 ID（已弃用，请使用 `resume_source_id`）。
    pub fn init_source_id(mut self, value: impl Into<String>) -> Self {
        self.init_source_id = Some(value.into());
        self
    }

    /// 设置简历来源 ID。
    pub fn resume_source_id(mut self, value: impl Into<String>) -> Self {
        self.resume_source_id = Some(value.into());
        self
    }

    /// 设置文件夹 ID 列表。
    pub fn folder_id_list(mut self, value: Vec<String>) -> Self {
        self.folder_id_list = Some(value);
        self
    }

    /// 设置更新人 ID。
    pub fn operator_id(mut self, value: impl Into<String>) -> Self {
        self.operator_id = Some(value.into());
        self
    }

    /// 设置更新人类型。
    pub fn operator_account_type(mut self, value: i32) -> Self {
        self.operator_account_type = Some(value);
        self
    }

    /// 设置简历附件 ID。
    pub fn resume_attachment_id(mut self, value: impl Into<String>) -> Self {
        self.resume_attachment_id = Some(value.into());
        self
    }

    /// 设置教育经历列表。
    pub fn education_list(mut self, value: Vec<TalentCombinedEducationInfo>) -> Self {
        self.education_list = Some(value);
        self
    }

    /// 设置工作经历列表。
    pub fn career_list(mut self, value: Vec<TalentCombinedCareerInfo>) -> Self {
        self.career_list = Some(value);
        self
    }

    /// 设置项目经历列表。
    pub fn project_list(mut self, value: Vec<TalentCombinedProjectInfo>) -> Self {
        self.project_list = Some(value);
        self
    }

    /// 设置作品列表。
    pub fn works_list(mut self, value: Vec<TalentCombinedWorkInfo>) -> Self {
        self.works_list = Some(value);
        self
    }

    /// 设置获奖列表。
    pub fn award_list(mut self, value: Vec<TalentCombinedAwardInfo>) -> Self {
        self.award_list = Some(value);
        self
    }

    /// 设置语言能力列表。
    pub fn language_list(mut self, value: Vec<TalentCombinedLanguageInfo>) -> Self {
        self.language_list = Some(value);
        self
    }

    /// 设置社交账号列表。
    pub fn sns_list(mut self, value: Vec<TalentCombinedSnsInfo>) -> Self {
        self.sns_list = Some(value);
        self
    }

    /// 设置意向地点列表。
    pub fn preferred_city_code_list(mut self, value: Vec<String>) -> Self {
        self.preferred_city_code_list = Some(value);
        self
    }

    /// 设置自我评价。
    pub fn self_evaluation(mut self, value: TalentSelfEvaluation) -> Self {
        self.self_evaluation = Some(value);
        self
    }

    /// 设置自定义模块列表。
    pub fn customized_data(mut self, value: Vec<TalentCustomizedDataObjectValue>) -> Self {
        self.customized_data = Some(value);
        self
    }
}

/// 综合更新候选人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedUpdateResponse {
    /// 更新结果
    pub result: bool,
}

// 批量获取候选人ID相关模型

/// 批量获取候选人ID请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdRequestBody {
    /// 候选人 ID 列表（必填，最多100个）
    pub talent_ids: Vec<String>,
}

/// 候选人ID映射信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalentIdInfo {
    /// 候选人 ID
    pub talent_id: String,
    /// 是否有效
    pub is_valid: bool,
}

/// 批量获取候选人ID响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetIdResponse {
    /// 候选人ID列表
    pub talent_id_list: Vec<TalentIdInfo>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_talent_serialization_roundtrip() {
        let talent = Talent {
            talent_id: "talent_1".to_string(),
            name: Some("张三".to_string()),
            resume: Some("<p>简历</p>".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            phone: Some("13800000000".to_string()),
            status: Some(2),
            create_time: Some(1700000000000),
            update_time: Some(1700000001000),
        };

        let text = serde_json::to_string(&talent).expect("序列化失败");
        let parsed: Talent = serde_json::from_str(&text).expect("反序列化失败");
        assert_eq!(parsed.talent_id, "talent_1");
        assert_eq!(parsed.status, Some(2));
    }

    #[test]
    fn test_talent_response_deserialization() {
        let response: BatchGetIdResponse = serde_json::from_str(
            r#"{"talent_id_list":[{"talent_id":"talent_2","is_valid":true}]}"#,
        )
        .expect("反序列化失败");

        assert_eq!(response.talent_id_list.len(), 1);
        assert!(response.talent_id_list[0].is_valid);
    }
}
