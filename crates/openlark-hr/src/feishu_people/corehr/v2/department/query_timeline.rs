//! 查询指定生效日期的部门基本信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 查询指定生效日期的部门基本信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryTimelineRequest {
    /// 部门 ID
    pub department_id: String,
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl QueryTimelineRequest {
    /// 创建请求
    pub fn new(department_id: String) -> Self {
        Self {
            department_id,
            effective_time: None,
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = Some(effective_time);
        self
    }
}

/// 查询指定生效日期的部门基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTimelineResponse {
    /// 部门信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentVersion>>,
}

/// 部门版本信息（department_timeline）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentVersion {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 部门版本 ID（默认返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 部门名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 部门类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    /// 上级部门 ID（查询一级部门时不展示）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人（员工雇佣 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 当前版本生效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 当前版本失效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 树形排序，同层级部门排序序号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    /// 列表排序，上级路径上所有 tree_order 用 "-" 拼接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    /// 是否根部门（默认返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    /// 是否保密（该功能暂不支持，可以忽略）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    /// 岗职管理模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
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

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（中文 zh-CN，英文 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段数据
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

/// 自定义字段名称：{zh_cn, en_us}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

impl ApiResponseTrait for QueryTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询指定生效日期的部门基本信息请求构建器
#[derive(Debug, Clone)]
pub struct QueryTimelineRequestBuilder {
    config: Config,
    request: QueryTimelineRequest,
}

impl QueryTimelineRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_id: String) -> Self {
        Self {
            config,
            request: QueryTimelineRequest::new(department_id),
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.request = self.request.effective_time(effective_time);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryTimelineResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentQueryTimeline;
        let request = ApiRequest::<QueryTimelineResponse>::post(api_endpoint.to_url());

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
            "查询指定生效日期的部门基本信息响应数据为空",
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

    /// 端到端：POST /open-apis/corehr/v2/departments/query_timeline
    #[tokio::test]
    async fn test_query_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/query_timeline"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "id": "4719456877659520852",
                        "version_id": "7238516215202170412",
                        "names": [{"lang": "zh-CN", "value": "中文示例"}],
                        "sub_type": {
                            "enum_name": "phone_type",
                            "display": [{"lang": "zh-CN", "value": "中文示例"}]
                        },
                        "parent_department_id": "4719456877659520111",
                        "manager": "6893013238632416777",
                        "code": "D00000456",
                        "effective_date": "2020-05-01",
                        "active": true,
                        "expiration_date": "2020-05-02",
                        "tree_order": "001000",
                        "list_order": "001000-001000",
                        "is_root": false,
                        "is_confidential": false,
                        "staffing_model": {
                            "enum_name": "phone_type",
                            "display": [{"lang": "zh-CN", "value": "中文示例"}]
                        },
                        "cost_center_id": "7142384817131652652"
                    }]
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

        let resp = QueryTimelineRequestBuilder::new(config, "test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.as_ref().expect("items 应被解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("4719456877659520852"));
        assert_eq!(items[0].version_id.as_deref(), Some("7238516215202170412"));
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|n| n.first())
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
        assert_eq!(items[0].effective_date.as_deref(), Some("2020-05-01"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
