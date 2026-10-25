//! 搜索部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 搜索部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
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

    /// 设置部门名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// 设置部门编码
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

/// 搜索部门信息响应
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的部门信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// `DepartmentItem`（官方 `department` 结构，除 `id` 外字段均受请求 `fields`
/// 参数与字段权限控制返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentItem {
    /// 部门 ID
    pub id: String,
    /// 部门记录版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 部门名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<Vec<I18nText>>,
    /// 部门类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    /// 上级部门 ID（需 corehr:department.organize:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人雇佣 ID（需 corehr:department.manager:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 树形排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    /// 列表排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否根部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    /// 是否保密
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    /// 当前版本生效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 当前版本失效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 自定义字段（需 corehr:department.custom_fields:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 岗职管理模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    /// 部门默认成本中心 ID（需 corehr:department.cost_center_id:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 创建时间（版本创建时间）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 更新人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// 记录创建时间（第一个版本的创建时间）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_created_time: Option<String>,
    /// 记录更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_updated_time: Option<String>,
    /// 记录创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_created_by: Option<String>,
    /// 记录更新人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_updated_by: Option<String>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言（如 zh-CN / en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
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

/// 搜索部门信息请求构建器
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

    /// 设置部门名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.request = self.request.parent_id(parent_id);
        self
    }

    /// 设置部门编码
    pub fn code(mut self, code: String) -> Self {
        self.request = self.request.code(code);
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
        let api_endpoint = FeishuPeopleApiV2::DepartmentSearch;
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
            "搜索部门信息响应数据为空",
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

    /// 端到端：POST /open-apis/corehr/v2/departments/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4719456877659520852",
                            "version_id": "6890452208593372611",
                            "department_name": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "sub_type": {
                                "enum_name": "phone_type",
                                "display": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ]
                            },
                            "parent_department_id": "4719456877659520111",
                            "manager": "6893013238632416777",
                            "tree_order": "001000",
                            "list_order": "001000-001000",
                            "code": "D00000456",
                            "is_root": false,
                            "is_confidential": false,
                            "effective_date": "2020-05-01",
                            "expiration_date": "2020-05-02",
                            "active": true,
                            "description": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "staffing_model": {
                                "enum_name": "position",
                                "display": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ]
                            },
                            "cost_center_id": "7142384817131652652",
                            "created_time": "2020-05-01 00:00:00",
                            "updated_time": "2020-05-02 00:00:00",
                            "created_by": "6893013238632416777",
                            "updated_by": "6893013238632416777"
                        }
                    ],
                    "page_token": "eyJldV9uYyI6IlswLFwiNjk2MTI4Njg0NjA5Mzc4ODY4MC03MjExMDM0ODcxMjA3OTUzOTc1XCJdIn0",
                    "has_more": true
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
            .expect("items 应包含部门");
        assert_eq!(item.id, "4719456877659520852");
        assert_eq!(item.version_id.as_deref(), Some("6890452208593372611"));
        assert_eq!(
            item.department_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );
        assert_eq!(
            item.sub_type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(
            item.parent_department_id.as_deref(),
            Some("4719456877659520111")
        );
        assert_eq!(item.manager.as_deref(), Some("6893013238632416777"));
        assert_eq!(item.active, Some(true));
        assert_eq!(item.created_time.as_deref(), Some("2020-05-01 00:00:00"));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
