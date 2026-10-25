//! 搜索员工信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employee/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 搜索员工信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置电话
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// 设置员工状态
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
}

/// 搜索员工信息响应
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的雇佣信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// `EmployeeItem`（官方 `employee` 结构）。
///
/// 官方响应由请求 `fields` 参数驱动（字段表下钻约 900 行，含 person_info
/// 嵌套等大量权限门控字段）；此处仅建模顶层平铺常用字段与 `job_level`
/// 容器，其余字段暂不建模。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeItem {
    /// 雇佣 ID，类型与 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 雇佣 ID（类型与 user_id_type 一致，转换失败时返回空值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id_v2: Option<String>,
    /// 招聘投递 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    /// 待入职 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prehire_id: Option<String>,
    /// 工号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 人员类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 人员子类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门 ID（类型与 department_id_type 一致，转换失败时返回空值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id_v2: Option<String>,
    /// 职级 ID（需 corehr:employment.job_level:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 岗位 ID（需 corehr:employment.position:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 职级（需 corehr:employment.job_level:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
}

/// 职级（官方 `employee.job_level` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobLevel {
    /// 职级 ID（与 job_level_id 值相同）
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

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：`{zh_cn, en_us}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 `custom_field_data` 结构。
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

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索员工信息请求构建器
#[derive(Debug, Clone)]
pub struct SearchRequestBuilder {
    config: Config,
    request: SearchRequest,
}

impl SearchRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: SearchRequest::new(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: String) -> Self {
        self.request = self.request.department_id(department_id);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: String) -> Self {
        self.request = self.request.email(email);
        self
    }

    /// 设置电话
    pub fn phone(mut self, phone: String) -> Self {
        self.request = self.request.phone(phone);
        self
    }

    /// 设置员工状态
    pub fn status(mut self, status: String) -> Self {
        self.request = self.request.status(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::EmployeeSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());

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
            "搜索员工信息响应数据为空",
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

    /// 端到端：POST /open-apis/corehr/v2/employees/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
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
                            "position_id": "6893014062142064135",
                            "job_level": {
                                "id": "4692446793125560154",
                                "level_order": 9999,
                                "code": "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY=",
                                "name": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "description": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "active": true
                            }
                        }
                    ],
                    "has_more": false
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

        let resp = SearchRequestBuilder::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含员工");
        assert_eq!(item.employment_id.as_deref(), Some("6893014062142064135"));
        assert_eq!(
            item.employment_id_v2.as_deref(),
            Some("6893014062142064135")
        );
        assert_eq!(
            item.ats_application_id.as_deref(),
            Some("6838119494196871234")
        );
        assert_eq!(item.prehire_id.as_deref(), Some("7023239238976141133"));
        assert_eq!(item.employee_number.as_deref(), Some("1000000"));
        assert_eq!(
            item.employee_type_id.as_deref(),
            Some("6971090097697521314")
        );
        assert_eq!(
            item.employee_subtype_id.as_deref(),
            Some("6971090097697521317")
        );
        assert_eq!(
            item.department_id_v2.as_deref(),
            Some("6893014062142064135")
        );
        assert_eq!(item.job_level_id.as_deref(), Some("6893014062142064135"));
        let job_level = item.job_level.as_ref().expect("job_level 应存在");
        assert_eq!(job_level.id.as_deref(), Some("4692446793125560154"));
        assert_eq!(job_level.level_order, Some(9999));
        assert_eq!(resp.has_more, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
