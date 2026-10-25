//! 批量查询员工信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employee/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 批量查询员工信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetRequest {
    /// 员工 ID 列表
    pub employee_ids: Vec<String>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(employee_ids: Vec<String>) -> Self {
        Self { employee_ids }
    }

    /// 添加员工 ID
    pub fn add_employee_id(mut self, employee_id: String) -> Self {
        self.employee_ids.push(employee_id);
        self
    }
}

/// 批量查询员工信息响应
///
/// 响应 `data` 子树形状：`{items: employee[]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 员工列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeItem>>,
}

/// `EmployeeItem`（官方 `employee` 结构的核心标识子集，其余字段后续按文档全表补齐）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeItem {
    /// 员工 ID（官方 `employee` 顶层无此字段，兼容保留；官方标识见 `employment_id`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 雇佣 ID，类型与 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 雇佣 ID（与 employment_id 一致，仅转换失败时返回空值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id_v2: Option<String>,
    /// 招聘投递 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    /// 待入职 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prehire_id: Option<String>,
    /// 工号（官方字段名）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 人员类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 人员子类型 ID（功能灰度中）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    /// 部门 ID（当按照 department_id_type 转换失败时返回为空值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id_v2: Option<String>,
    /// 职级 ID（需 corehr:employment.job_level:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级（需 corehr:employment.job_level:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<EmployeeJobLevel>,
}

/// 员工职级信息（官方 `employee.job_level` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeJobLevel {
    /// 职级 ID，与 job_level_id 值相同
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 职级数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i64>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：`{zh_cn, en_us}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 `custom_field_data` 结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询员工信息请求构建器
#[derive(Debug, Clone)]
pub struct BatchGetRequestBuilder {
    config: Config,
    request: BatchGetRequest,
}

impl BatchGetRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, employee_ids: Vec<String>) -> Self {
        Self {
            config,
            request: BatchGetRequest::new(employee_ids),
        }
    }

    /// 添加员工 ID
    pub fn add_employee_id(mut self, employee_id: String) -> Self {
        self.request = self.request.add_employee_id(employee_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::EmployeeBatchGet;
        let request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询员工信息响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/employees/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "6893014062142064135",
                            "employment_id_v2": "6893014062142064135",
                            "ats_application_id": "6838119494196871234",
                            "prehire_id": "7023239238976141133",
                            "employee_number": "1000000",
                            "employee_type_id": "6971090097697521314",
                            "employee_subtype_id": "6971090097697521317",
                            "department_id": "6893014062142064135",
                            "department_id_v2": "6893014062142064135",
                            "job_level_id": "6893014062142064135",
                            "job_level": {
                                "id": "4692446793125560154",
                                "level_order": 9999,
                                "code": "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY=",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }],
                                "description": [{ "lang": "zh-CN", "value": "中文示例" }],
                                "active": true,
                                "custom_fields": []
                            }
                        }
                    ]
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

        let resp = BatchGetRequestBuilder::new(config, vec!["test001".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含员工");
        assert_eq!(item.id, None);
        assert_eq!(item.employment_id.as_deref(), Some("6893014062142064135"));
        assert_eq!(item.employee_number.as_deref(), Some("1000000"));
        assert_eq!(
            item.department_id_v2.as_deref(),
            Some("6893014062142064135")
        );
        let job_level = item.job_level.as_ref().expect("应包含 job_level");
        assert_eq!(job_level.level_order, Some(9999));
        assert_eq!(job_level.active, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
