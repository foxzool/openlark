//! 批量获取员工花名册信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/ehr-v1/employee/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工花名册信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选，取值范围 1~100，默认 10）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 用户 ID 列表（可选，不超过 100 个）
    user_ids: Option<Vec<String>>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_ids: None,
            config,
        }
    }

    /// 设置分页大小（可选，取值范围 1~100，默认 10）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 列表（可选，不超过 100 个）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::EhrApiV1;

        // 1. 构建端点
        let api_endpoint = EhrApiV1::EmployeeList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(ref user_ids) = self.user_ids {
            let user_ids_str = user_ids.join(",");
            request = request.query("user_ids", &user_ids_str);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取员工花名册响应数据为空",
        )
        .await
    }
}

/// 批量获取员工花名册信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 员工花名册信息列表
    pub items: Vec<EmployeeProfile>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 员工花名册信息（官方 item 顶层仅 user_id + system_fields + custom_fields）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeProfile {
    /// 员工的用户 ID（「待入职」和「已取消入职」的员工，此字段值为 null）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 系统字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fields: Option<SystemFields>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 系统字段（view=full 时返回；view=basic 仅返回部分字段，故全部 Option 化）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemFields {
    /// 中文姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 部门的飞书 open_department_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<ManagerRef>,
    /// 职位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<IdNameObject>,
    /// 职级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<IdNameObject>,
    /// 工作地点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location: Option<IdNameObject>,
    /// 性别：1-男 2-女
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 出生日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// 籍贯
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_region: Option<NativeRegion>,
    /// 民族
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<i32>,
    /// 婚姻状况：1-未婚 2-已婚 3-离异 4-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i32>,
    /// 政治面貌
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub political_status: Option<i32>,
    /// 参加工作日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entered_workforce_date: Option<String>,
    /// 证件类型：1-居民身份证 2-港澳居民来往内地通行证 3-台湾居民来往大陆通行证 4-护照 5-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<i32>,
    /// 证件号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// 户口类型：1-本市城镇 2-外埠城镇 3-本市农村 4-外埠农村
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_type: Option<i32>,
    /// 户口所在地
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_location: Option<String>,
    /// 银行卡号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    /// 开户行
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 社保账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_account: Option<String>,
    /// 公积金账号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provident_fund_account: Option<String>,
    /// 工号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 人员类型：1-正式 2-实习 3-顾问 4-外包 5-劳务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    /// 员工状态：1-待入职 2-在职 3-已取消入职 4-待离职 5-已离职
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 入职日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 试用期（月）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_months: Option<f64>,
    /// 转正日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion_date: Option<String>,
    /// 转正申请：1-未申请 2-审批中 3-被驳回 4-已通过
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<i32>,
    /// 转正状态：1-无需转正 2-待转正 3-已转正
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_status: Option<i32>,
    /// 离职日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_day: Option<String>,
    /// 离职类型：1-主动 2-被动 3-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_type: Option<i32>,
    /// 离职原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_reason: Option<i32>,
    /// 离职备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_notes: Option<String>,
    /// 合同公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_company: Option<IdNameObject>,
    /// 合同类型：1-固定期限劳动合同 2-无固定期限劳动合同 3-实习协议 4-外包协议 5-劳务派遣合同 6-返聘协议 7-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<i32>,
    /// 合同开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    /// 合同到期日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_expiration_date: Option<String>,
    /// 劳动合同签订次数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_sign_times: Option<i32>,
    /// 个人邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_email: Option<String>,
    /// 家庭地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_address: Option<String>,
    /// 主要紧急联系人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_emergency_contact: Option<EmergencyContact>,
    /// 紧急联系人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<Vec<EmergencyContact>>,
    /// 最高学历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_level_of_edu: Option<Education>,
    /// 教育经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<Education>>,
    /// 前工作经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub former_work_exp: Option<WorkExperience>,
    /// 工作经历
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_exp: Option<Vec<WorkExperience>>,
    /// 身份证照片（人像面）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_photo_po_side: Option<Vec<EmployeeAttachment>>,
    /// 身份证照片（国徽面）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_photo_em_side: Option<Vec<EmployeeAttachment>>,
    /// 证件照
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_photo: Option<Vec<EmployeeAttachment>>,
    /// 学位证书
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diploma_photo: Option<Vec<EmployeeAttachment>>,
    /// 毕业证书
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graduation_cert: Option<Vec<EmployeeAttachment>>,
    /// 奖励证明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_of_merit: Option<Vec<EmployeeAttachment>>,
    /// 离职证明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_file: Option<Vec<EmployeeAttachment>>,
    /// 取消入职原因：1-个人原因 2-原单位留任 3-接受其他 Offer 4-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_onboarding_reason: Option<i32>,
    /// 取消入职备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_onboarding_notes: Option<String>,
    /// 入职登记表状态：1-未发送 2-待提交 3-已提交
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_form_status: Option<i32>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

/// 上级（官方 manager 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManagerRef {
    /// 上级的用户 ID（user_id）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 中文名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
}

/// {id, name} 同构对象（job / job_level / work_location / contract_company）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdNameObject {
    /// ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 籍贯（官方 native_region 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NativeRegion {
    /// ISO 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_code: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 紧急联系人（官方 emergency_contact 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmergencyContact {
    /// 紧急联系人姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 与紧急联系人的关系：1-父母 2-配偶 3-子女 4-兄弟姐妹 5-朋友 6-其他
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<i32>,
    /// 手机号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

/// 教育经历（官方 education 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Education {
    /// 学历：1-小学 2-初中 3-高中 4-职业高级中学 5-中等专业学校 6-大专 7-本科 8-硕士 9-博士
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 毕业学校
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 专业
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学位：1-学士 2-硕士 3-博士
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<i32>,
    /// 开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// 结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// 工作经历（官方 work_experience 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkExperience {
    /// 公司
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 职位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    /// 开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// 截止日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 工作描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 附件（官方 attachment 对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeAttachment {
    /// 下载文件所需要的 Token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 文件类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 大小
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// 自定义字段（官方 custom_fields 数组元素）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomField {
    /// 自定义字段 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 自定义字段类型：text-文本 date-日期 option-枚举 file-附件
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 字段值（根据 type 不同，结构不同）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_list_employees_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：items[].user_id + system_fields + custom_fields（.md L192-386）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "user_id": "ou_db362c0e79f5a26db1ca8e94698ee417",
                        "system_fields": {
                            "name": "张三",
                            "en_name": "Tony Zhang",
                            "email": "a@b.com",
                            "mobile": "(+86) 13812345678",
                            "department_id": "od-4d551617a5da3cec26666d33175dc8ca",
                            "manager": {
                                "user_id": "ou_db362c0e79f5a26db1ca8e94698ee417",
                                "name": "李四",
                                "en_name": "Tom Li"
                            },
                            "job": {
                                "id": 1695838220091399,
                                "name": "测试工程师"
                            },
                            "job_level": {
                                "id": 1695838220091399,
                                "name": "CEO"
                            },
                            "work_location": {
                                "id": 1695838220091399,
                                "name": "武汉"
                            },
                            "gender": 1,
                            "birthday": "2020-01-01",
                            "native_region": {
                                "iso_code": "CHN-11",
                                "name": "北京"
                            },
                            "ethnicity": 2,
                            "marital_status": 2,
                            "political_status": 2,
                            "entered_workforce_date": "2020-01-01",
                            "id_type": 1,
                            "id_number": "110122XXXXXX",
                            "hukou_type": 1,
                            "hukou_location": "北京市海淀区XXXX",
                            "bank_account_number": "1243253453",
                            "bank_name": "招商银行",
                            "social_security_account": "123124124",
                            "provident_fund_account": "123124235",
                            "employee_no": "TM-00001",
                            "employee_type": 1,
                            "status": 2,
                            "hire_date": "2020-01-01",
                            "probation_months": 2,
                            "conversion_date": "2020-01-01",
                            "application": 1,
                            "application_status": 2,
                            "last_day": "2020-01-01",
                            "departure_type": 1,
                            "departure_reason": 4,
                            "departure_notes": "世界那么大",
                            "contract_company": {
                                "id": 1695838220091399,
                                "name": "油条一号"
                            },
                            "contract_type": 1,
                            "contract_start_date": "2020-01-01",
                            "contract_expiration_date": "2020-01-01",
                            "contract_sign_times": 2,
                            "personal_email": "personal@email.com",
                            "family_address": "北京市海淀区XXXXX",
                            "primary_emergency_contact": {
                                "name": "张三",
                                "relationship": 1,
                                "mobile": "(+86) 13812345678"
                            },
                            "emergency_contact": [
                                {
                                    "name": "张三",
                                    "relationship": 1,
                                    "mobile": "(+86) 13812345678"
                                }
                            ],
                            "highest_level_of_edu": {
                                "level": 8,
                                "school": "XXXX大学",
                                "major": "XXX专业",
                                "degree": 2,
                                "start": "2020-01-01",
                                "end": "2020-01-01"
                            },
                            "education": [
                                {
                                    "level": 8,
                                    "school": "XXXX大学",
                                    "major": "XXX专业",
                                    "degree": 2,
                                    "start": "2020-01-01",
                                    "end": "2020-01-01"
                                }
                            ],
                            "former_work_exp": {
                                "company": "XXXX公司",
                                "department": "部门1",
                                "job": "职位",
                                "start": "2020-01-01",
                                "end": "2020-01-01",
                                "description": "工作描述"
                            },
                            "work_exp": [
                                {
                                    "company": "XXXX公司",
                                    "department": "部门1",
                                    "job": "职位",
                                    "start": "2020-01-01",
                                    "end": "2020-01-01",
                                    "description": "工作描述"
                                }
                            ],
                            "id_photo_po_side": [
                                {
                                    "id": "c7273e07ed9e40a394f88c7dccb49212",
                                    "mime_type": "png",
                                    "name": "Custom Code.png",
                                    "size": 57380
                                }
                            ],
                            "offboarding_file": [
                                {
                                    "id": "c7273e07ed9e40a394f88c7dccb49212",
                                    "mime_type": "png",
                                    "name": "Custom Code.png",
                                    "size": 57380
                                }
                            ],
                            "cancel_onboarding_reason": 2,
                            "cancel_onboarding_notes": "个人原因",
                            "employee_form_status": 1,
                            "create_time": 1608690517811,
                            "update_time": 1608690517811
                        },
                        "custom_fields": [
                            {
                                "key": "field_xxxxxxxx",
                                "label": "自定义字段 1",
                                "type": "date",
                                "value": "2021-01-13"
                            }
                        ]
                    }
                ],
                "page_token": "10",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/employees"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ListRequest::new(config)
            .execute()
            .await
            .expect("批量获取员工花名册应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        let profile = &data.items[0];
        assert_eq!(
            profile.user_id.as_deref(),
            Some("ou_db362c0e79f5a26db1ca8e94698ee417")
        );
        let system_fields = profile
            .system_fields
            .as_ref()
            .expect("system_fields 应反序列化");
        assert_eq!(system_fields.name.as_deref(), Some("张三"));
        assert_eq!(system_fields.en_name.as_deref(), Some("Tony Zhang"));
        assert_eq!(system_fields.email.as_deref(), Some("a@b.com"));
        assert_eq!(system_fields.mobile.as_deref(), Some("(+86) 13812345678"));
        assert_eq!(
            system_fields.department_id.as_deref(),
            Some("od-4d551617a5da3cec26666d33175dc8ca")
        );
        assert_eq!(
            system_fields
                .manager
                .as_ref()
                .and_then(|m| m.en_name.as_deref()),
            Some("Tom Li")
        );
        assert_eq!(
            system_fields.job.as_ref().and_then(|j| j.id),
            Some(1695838220091399)
        );
        assert_eq!(system_fields.hire_date.as_deref(), Some("2020-01-01"));
        assert_eq!(system_fields.create_time, Some(1608690517811));
        let custom_fields = profile
            .custom_fields
            .as_ref()
            .expect("custom_fields 应反序列化");
        assert_eq!(custom_fields[0].key.as_deref(), Some("field_xxxxxxxx"));
        assert_eq!(custom_fields[0].r#type.as_deref(), Some("date"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/ehr/v1/employees");
    }
}
