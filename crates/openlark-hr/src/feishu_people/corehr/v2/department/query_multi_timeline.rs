//! 批量查询部门版本信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 批量查询部门版本信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineRequest {
    /// 部门 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl QueryMultiTimelineRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置部门 ID 列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);
        self
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
}

/// 批量查询部门版本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMultiTimelineResponse {
    /// 部门信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentTimelineItem>>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 部门版本信息（官方 department_timeline 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentTimelineItem {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 部门版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 部门名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 部门子类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    /// 上级部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 版本生效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 版本失效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 树形排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    /// 列表排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    /// 是否根部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    /// 是否保密
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    /// 岗职管理模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言，中文用 zh-CN，英文用 en-US
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 自定义字段名称（官方 custom_name 结构：{zh_cn, en_us}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 自定义字段（官方 custom_field_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for QueryMultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门版本信息请求构建器
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequestBuilder {
    config: Config,
    request: QueryMultiTimelineRequest,
}

impl QueryMultiTimelineRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: QueryMultiTimelineRequest::new(),
        }
    }

    /// 设置部门 ID 列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request = self.request.department_ids(department_ids);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: String) -> Self {
        self.request = self.request.start_time(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: String) -> Self {
        self.request = self.request.end_time(end_time);
        self
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentQueryMultiTimeline;
        let request = ApiRequest::<QueryMultiTimelineResponse>::post(api_endpoint.to_url());

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
            "批量查询部门版本信息响应数据为空",
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

    /// 端到端：POST /open-apis/corehr/v2/departments/query_multi_timeline
    #[tokio::test]
    async fn test_query_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/departments/query_multi_timeline",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4719456877659520852",
                            "version_id": "7238516215202170412",
                            "names": [
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
                            "code": "D00000456",
                            "effective_date": "2020-05-01",
                            "active": true,
                            "expiration_date": "2020-05-02",
                            "is_root": false
                        }
                    ],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
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

        let resp = QueryMultiTimelineRequestBuilder::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].version_id.as_deref(), Some("7238516215202170412"));
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|names| names.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );
        assert_eq!(
            items[0]
                .sub_type
                .as_ref()
                .and_then(|s| s.enum_name.as_deref()),
            Some("phone_type")
        );
        assert_eq!(
            items[0].parent_department_id.as_deref(),
            Some("4719456877659520111")
        );
        assert_eq!(items[0].effective_date.as_deref(), Some("2020-05-01"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
