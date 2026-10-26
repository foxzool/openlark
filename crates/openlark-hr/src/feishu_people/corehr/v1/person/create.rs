//! 创建个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::{
    Address, BankAccount, CreateRequestBody, Dependent, Education, Email, EmergencyContact,
    EnumValue, NationalId, ObjectFieldData, PersonName, PersonalProfile, Phone, WorkExperience,
};

/// 创建个人信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 姓名（必填）
    name_list: Vec<PersonName>,
    /// 性别
    gender: Option<EnumValue>,
    /// 出生日期（格式：YYYY-MM-DD）
    date_of_birth: Option<String>,
    /// 国籍 ID
    nationality_id: Option<String>,
    /// 民族 / 种族
    race: Option<EnumValue>,
    /// 婚姻状况
    marital_status: Option<EnumValue>,
    /// 电话列表
    phone_list: Option<Vec<Phone>>,
    /// 地址列表
    address_list: Option<Vec<Address>>,
    /// 邮箱列表
    email_list: Option<Vec<Email>>,
    /// 工作履历
    work_experience_list: Option<Vec<WorkExperience>>,
    /// 教育经历
    education_list: Option<Vec<Education>>,
    /// 银行账号
    bank_account_list: Option<Vec<BankAccount>>,
    /// 证件号码
    national_id_list: Option<Vec<NationalId>>,
    /// 亲属
    dependent_list: Option<Vec<Dependent>>,
    /// 紧急联系人
    emergency_contact_list: Option<Vec<EmergencyContact>>,
    /// 参加工作日期（格式：YYYY-MM-DD）
    date_entered_workforce: Option<String>,
    /// 头像资源的 id
    profile_image_id: Option<String>,
    /// 自定义字段
    custom_fields: Option<Vec<ObjectFieldData>>,
    /// 纳税身份信息
    resident_tax_id_list: Option<Vec<String>>,
    /// 年龄
    age: Option<i64>,
    /// 个人资料
    personal_profile: Option<Vec<PersonalProfile>>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name_list: Vec::new(),
            gender: None,
            date_of_birth: None,
            nationality_id: None,
            race: None,
            marital_status: None,
            phone_list: None,
            address_list: None,
            email_list: None,
            work_experience_list: None,
            education_list: None,
            bank_account_list: None,
            national_id_list: None,
            dependent_list: None,
            emergency_contact_list: None,
            date_entered_workforce: None,
            profile_image_id: None,
            custom_fields: None,
            resident_tax_id_list: None,
            age: None,
            personal_profile: None,
        }
    }

    /// 设置姓名（必填）
    pub fn name_list(mut self, name_list: Vec<PersonName>) -> Self {
        self.name_list = name_list;
        self
    }

    /// 添加一条姓名
    pub fn name(mut self, name: PersonName) -> Self {
        self.name_list.push(name);
        self
    }

    /// 设置性别
    pub fn gender(mut self, gender: EnumValue) -> Self {
        self.gender = Some(gender);
        self
    }

    /// 设置出生日期（格式：YYYY-MM-DD）
    pub fn date_of_birth(mut self, date_of_birth: String) -> Self {
        self.date_of_birth = Some(date_of_birth);
        self
    }

    /// 设置国籍 ID
    pub fn nationality_id(mut self, nationality_id: String) -> Self {
        self.nationality_id = Some(nationality_id);
        self
    }

    /// 设置民族 / 种族
    pub fn race(mut self, race: EnumValue) -> Self {
        self.race = Some(race);
        self
    }

    /// 设置婚姻状况
    pub fn marital_status(mut self, marital_status: EnumValue) -> Self {
        self.marital_status = Some(marital_status);
        self
    }

    /// 设置电话列表
    pub fn phone_list(mut self, phone_list: Vec<Phone>) -> Self {
        self.phone_list = Some(phone_list);
        self
    }

    /// 设置地址列表
    pub fn address_list(mut self, address_list: Vec<Address>) -> Self {
        self.address_list = Some(address_list);
        self
    }

    /// 设置邮箱列表
    pub fn email_list(mut self, email_list: Vec<Email>) -> Self {
        self.email_list = Some(email_list);
        self
    }

    /// 设置工作履历
    pub fn work_experience_list(mut self, work_experience_list: Vec<WorkExperience>) -> Self {
        self.work_experience_list = Some(work_experience_list);
        self
    }

    /// 设置教育经历
    pub fn education_list(mut self, education_list: Vec<Education>) -> Self {
        self.education_list = Some(education_list);
        self
    }

    /// 设置银行账号
    pub fn bank_account_list(mut self, bank_account_list: Vec<BankAccount>) -> Self {
        self.bank_account_list = Some(bank_account_list);
        self
    }

    /// 设置证件号码
    pub fn national_id_list(mut self, national_id_list: Vec<NationalId>) -> Self {
        self.national_id_list = Some(national_id_list);
        self
    }

    /// 设置亲属
    pub fn dependent_list(mut self, dependent_list: Vec<Dependent>) -> Self {
        self.dependent_list = Some(dependent_list);
        self
    }

    /// 设置紧急联系人
    pub fn emergency_contact_list(mut self, emergency_contact_list: Vec<EmergencyContact>) -> Self {
        self.emergency_contact_list = Some(emergency_contact_list);
        self
    }

    /// 设置参加工作日期（格式：YYYY-MM-DD）
    pub fn date_entered_workforce(mut self, date_entered_workforce: String) -> Self {
        self.date_entered_workforce = Some(date_entered_workforce);
        self
    }

    /// 设置头像资源的 id
    pub fn profile_image_id(mut self, profile_image_id: String) -> Self {
        self.profile_image_id = Some(profile_image_id);
        self
    }

    /// 设置自定义字段
    pub fn custom_fields(mut self, custom_fields: Vec<ObjectFieldData>) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }

    /// 设置纳税身份信息
    pub fn resident_tax_id_list(mut self, resident_tax_id_list: Vec<String>) -> Self {
        self.resident_tax_id_list = Some(resident_tax_id_list);
        self
    }

    /// 设置年龄
    pub fn age(mut self, age: i64) -> Self {
        self.age = Some(age);
        self
    }

    /// 设置个人资料
    pub fn personal_profile(mut self, personal_profile: Vec<PersonalProfile>) -> Self {
        self.personal_profile = Some(personal_profile);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.name_list, "姓名（name_list）不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::PersonCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体（顶层平铺，官方无 person 信封）
        let request_body = CreateRequestBody {
            name_list: self.name_list,
            gender: self.gender,
            date_of_birth: self.date_of_birth,
            nationality_id: self.nationality_id,
            race: self.race,
            marital_status: self.marital_status,
            phone_list: self.phone_list,
            address_list: self.address_list,
            email_list: self.email_list,
            work_experience_list: self.work_experience_list,
            education_list: self.education_list,
            bank_account_list: self.bank_account_list,
            national_id_list: self.national_id_list,
            dependent_list: self.dependent_list,
            emergency_contact_list: self.emergency_contact_list,
            date_entered_workforce: self.date_entered_workforce,
            profile_image_id: self.profile_image_id,
            custom_fields: self.custom_fields,
            resident_tax_id_list: self.resident_tax_id_list,
            age: self.age,
            personal_profile: self.personal_profile,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建个人信息响应数据为空",
        )
        .await
    }
}

/// 创建个人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 个人信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<super::models::Person>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/persons
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/persons"))
            .and(body_partial_json(json!({
                // 官方形状：name_list 顶层平铺，无 person 信封
                "name_list": [{
                    "local_primary": "黄",
                    "local_first_name": "四",
                    "country_region_id": "6862995757234914824",
                    "name_type": { "enum_name": "type_1" }
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "person": {
                        "phone_number": "11111111111",
                        "legal_name": "张三",
                        "preferred_name": "张四",
                        "id": "646465654545",
                        "name_list": [{
                            "local_primary": "黄",
                            "local_first_name": "四",
                            "country_region_id": "6862995757234914824",
                            "name_type": {
                                "enum_name": "type_1",
                                "display": [{ "lang": "zh-CN", "value": "刘梓新" }]
                            }
                        }]
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

        let data = CreateRequest::new(config)
            .name(PersonName {
                local_primary: Some("黄".to_string()),
                local_first_name: Some("四".to_string()),
                country_region_id: Some("6862995757234914824".to_string()),
                name_type: Some(EnumValue {
                    enum_name: Some("type_1".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .execute()
            .await
            .expect("请求应成功");

        let person = data.person.expect("person 应存在");
        assert_eq!(person.phone_number, Some("11111111111".to_string()));
        assert_eq!(person.legal_name, Some("张三".to_string()));
        assert_eq!(person.preferred_name, Some("张四".to_string()));
        assert_eq!(person.id, Some("646465654545".to_string()));
        let name_list = person.name_list.expect("name_list 应存在");
        assert_eq!(name_list.len(), 1);
        assert_eq!(name_list[0].local_first_name, Some("四".to_string()));
        assert_eq!(
            name_list[0].country_region_id,
            Some("6862995757234914824".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    /// 校验：未设置 name_list 时执行失败
    #[tokio::test]
    async fn test_create_validation_fails_without_name_list() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .enable_token_cache(false)
            .build();

        let result = CreateRequest::new(config).execute().await;
        assert!(result.is_err());
    }
}
