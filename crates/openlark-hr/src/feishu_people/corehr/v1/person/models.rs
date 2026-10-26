//! CoreHR 个人信息（person）相关模型
//!
//! 字段表对齐官方文档（查询/创建/更新接口共用）。

use serde::{Deserialize, Serialize};

/// 个人信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Person {
    /// 个人手机号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// 法定姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// 常用名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// 实体在 CoreHR 内部的唯一键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_list: Option<Vec<PersonName>>,
    /// 性别
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<EnumValue>,
    /// 出生日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// 国籍 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 民族 / 种族
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<EnumValue>,
    /// 婚姻状况
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<EnumValue>,
    /// 电话列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_list: Option<Vec<Phone>>,
    /// 地址列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<Address>>,
    /// 邮件列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<Email>>,
    /// 工作履历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<WorkExperience>>,
    /// 教育经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<Education>>,
    /// 银行账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_list: Option<Vec<BankAccount>>,
    /// 证件号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_list: Option<Vec<NationalId>>,
    /// 亲属
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_list: Option<Vec<Dependent>>,
    /// 紧急联系人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
    /// 参加工作日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_entered_workforce: Option<String>,
    /// 头像资源的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_image_id: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// 纳税身份信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resident_tax_id_list: Option<Vec<String>>,
    /// 年龄
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    /// 最高学历教育
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_level_of_education: Option<Education>,
    /// 最高学位教育经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_degree_of_education: Option<Education>,
    /// 个人资料
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile: Option<Vec<PersonalProfile>>,
}

/// 姓名（person_name）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PersonName {
    /// 姓 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    /// 名 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    /// 国家 / 地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<EnumValue>,
    /// 名 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name_2: Option<String>,
    /// 姓 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary_2: Option<String>,
    /// 补充姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name_type: Option<EnumValue>,
    /// 名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// 全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// 姓氏称谓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hereditary: Option<String>,
    /// 自定义姓名（未传入时按所属国家 / 地区规则拼接）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// 本地文字的自定义姓名（未传入时按所属国家 / 地区规则拼接）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_local_name: Option<String>,
    /// 中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// 姓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_primary: Option<String>,
    /// 第二姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
    /// 尊称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social: Option<EnumValue>,
    /// 婚后姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<String>,
    /// 头衔
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<EnumValue>,
    /// 本地中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_middle_name: Option<String>,
    /// 第二姓氏 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_secondary: Option<String>,
    /// 展示姓名（本地和西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_and_western_script: Option<String>,
    /// 展示姓名（本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_script: Option<String>,
    /// 展示姓名（西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_western_script: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 电话（phone）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Phone {
    /// 国家区号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_area_code: Option<EnumValue>,
    /// 电话号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// 完整电话号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_phone_number: Option<String>,
    /// 设备类型（mobile_phone：手机；landline：座机；fax：传真）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<EnumValue>,
    /// 电话用途（home：家庭；work：工作；emergency_contact：紧急联系人；company：公司）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_usage: Option<EnumValue>,
    /// 主要电话
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 公开电话
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 地址（address）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Address {
    /// 完整地址（本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_local_script: Option<String>,
    /// 完整地址（西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_western_script: Option<String>,
    /// 地址 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 国家 / 地区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 主要行政区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 城市
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_id: Option<String>,
    /// 区/县
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_id: Option<String>,
    /// 地址行 1（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line1: Option<String>,
    /// 地址行 2（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line2: Option<String>,
    /// 地址行 3（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line3: Option<String>,
    /// 地址行 4（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line4: Option<String>,
    /// 地址行 5（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line5: Option<String>,
    /// 地址行 6（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line6: Option<String>,
    /// 地址行 7（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line7: Option<String>,
    /// 地址行 8（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line8: Option<String>,
    /// 地址行 9（非拉丁语系的本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line9: Option<String>,
    /// 邮政编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// 地址类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_type_list: Option<Vec<EnumValue>>,
    /// 主要地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 公开地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 邮件（email）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Email {
    /// 邮箱号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 主要邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 公开邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// 邮箱用途
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_usage: Option<EnumValue>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 工作履历（work_experience）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WorkExperience {
    /// 公司 / 组织
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_organization: Option<Vec<I18n>>,
    /// 部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<I18n>>,
    /// 岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<Vec<I18n>>,
    /// 工作描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18n>>,
    /// 开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 教育经历（education）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Education {
    /// 学校
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<Vec<I18n>>,
    /// 学历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_of_education: Option<EnumValue>,
    /// 开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 专业
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<Vec<I18n>>,
    /// 学位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<EnumValue>,
    /// 学校名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_name: Option<EnumValue>,
    /// 专业名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_of_study_name: Option<EnumValue>,
    /// 国家地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 预期结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_end_date: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 银行账号（bank_account）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BankAccount {
    /// 银行名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 银行账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    /// 开户人姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<String>,
    /// 银行名称（该字段待作废，请勿使用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<EnumValue>,
    /// 支行名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 国家/地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 银行卡用途
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_usage: Option<Vec<EnumValue>>,
    /// 银行卡类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<EnumValue>,
    /// 货币 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
}

/// 证件号码（national_id）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct NationalId {
    /// 国家证件类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_type_id: Option<String>,
    /// 证件号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    /// 证件签发日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// 证件到期日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 国家 / 地区
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 证件签发机构
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 亲属（dependent）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Dependent {
    /// 姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    /// 关系
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<EnumValue>,
    /// 性别
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<EnumValue>,
    /// 生日
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// 国籍 ID（该字段已作废，请使用 nationality_id_v2 字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 证件号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_list: Option<Vec<NationalId>>,
    /// 配偶工作状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spouses_working_status: Option<EnumValue>,
    /// 包含家属医疗保险
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_covered_by_health_insurance: Option<bool>,
    /// 允许家属抵扣税款
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_this_person_allowed_for_tax_deduction: Option<bool>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 家庭成员姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_name: Option<String>,
    /// 工作单位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    /// 岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    /// 电话
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<Phone>,
    /// 联系地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// 出生证明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_certificate_of_child: Option<Vec<PersonFile>>,
}

/// 紧急联系人（emergency_contact）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EmergencyContact {
    /// 姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PersonName>,
    /// 关系
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<EnumValue>,
    /// 电话（官方字段名为 phone_ist）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_ist: Option<Vec<Phone>>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 法定姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
}

/// 个人资料（personal_profile）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PersonalProfile {
    /// 个人资料 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_id: Option<String>,
    /// 资料类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile_type: Option<EnumValue>,
    /// 上传文件列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PersonFile>>,
}

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18n {
    /// 语言编码，如 `zh-CN`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 结构：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18n>>,
}

/// 官方自定义字段结构：`{field_name, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值（json 转义后的字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方文件结构：`{id}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PersonFile {
    /// 上传文件 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// 创建个人信息相关模型

/// 创建个人信息请求体
///
/// 顶层字段与官方文档对齐：`name_list` 必填，其余可选。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 姓名（必填）
    pub name_list: Vec<PersonName>,
    /// 性别
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<EnumValue>,
    /// 出生日期（格式：YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// 国籍 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    /// 民族 / 种族
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<EnumValue>,
    /// 婚姻状况
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<EnumValue>,
    /// 电话列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_list: Option<Vec<Phone>>,
    /// 地址列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<Address>>,
    /// 邮箱列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<Email>>,
    /// 工作履历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_experience_list: Option<Vec<WorkExperience>>,
    /// 教育经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<Education>>,
    /// 银行账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_list: Option<Vec<BankAccount>>,
    /// 证件号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub national_id_list: Option<Vec<NationalId>>,
    /// 亲属
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_list: Option<Vec<Dependent>>,
    /// 紧急联系人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
    /// 参加工作日期（格式：YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_entered_workforce: Option<String>,
    /// 头像资源的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_image_id: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 纳税身份信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resident_tax_id_list: Option<Vec<String>>,
    /// 年龄
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    /// 个人资料
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_profile: Option<Vec<PersonalProfile>>,
}
