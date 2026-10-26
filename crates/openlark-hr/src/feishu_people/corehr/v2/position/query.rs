//! 查询岗位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/position/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询岗位信息请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryBody {
    /// 部门 ID 列表（ID 类型需与 department_id_type 参数取值一致）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 版本生效日期，格式是 YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 启停用状态：true 为启用，false 为停用，不传则默认都返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 需要查询的字段列表，为空时仅返回 wk_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// 岗位 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_ids: Option<Vec<String>>,
    /// 岗位 Code 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_codes: Option<Vec<String>>,
}

/// 查询岗位信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: QueryBody,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页大小（必填，取值范围 1～100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: QueryBody::default(),
            department_id_type: None,
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: QueryBody) -> Self {
        self.body = body;
        self
    }

    /// 设置部门 ID 列表。
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.body.department_ids = Some(department_ids);
        self
    }

    /// 设置版本生效日期（格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = Some(effective_time.into());
        self
    }

    /// 设置启停用状态筛选。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = Some(active);
        self
    }

    /// 设置需要查询的字段列表。
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.body.fields = Some(fields);
        self
    }

    /// 设置岗位 ID 列表。
    pub fn position_ids(mut self, position_ids: Vec<String>) -> Self {
        self.body.position_ids = Some(position_ids);
        self
    }

    /// 设置岗位 Code 列表。
    pub fn position_codes(mut self, position_codes: Vec<String>) -> Self {
        self.body.position_codes = Some(position_codes);
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小（必填，取值范围 1～100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }

        let request = ApiRequest::<QueryResponse>::post("/open-apis/corehr/v2/positions/query")
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询岗位信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 岗位信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PositionItem>>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 岗位信息（官方 position 结构；字段受请求 fields 参数控制按需返回）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PositionItem {
    /// 岗位 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 状态，true 表示启用，false 表示停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 序列 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id_list: Option<Vec<String>>,
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职级 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id_list: Option<Vec<String>>,
    /// 人员类型 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id_list: Option<Vec<String>>,
    /// 职等 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id_list: Option<Vec<String>>,
    /// 工作地点 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id_list: Option<Vec<String>>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 直属上级岗位 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_leader_id: Option<String>,
    /// 虚线上级岗位 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_id: Option<String>,
    /// 是否关键岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_key_position: Option<bool>,
    /// 版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/positions/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/positions/query"))
            .and(query_param("page_size", "100"))
            .and(body_json(json!({
                "department_ids": ["7094136522860922111"],
                "effective_time": "2020-01-01",
                "active": true,
                "fields": ["name"],
                "position_ids": ["7094136582860923111"],
                "position_codes": ["42232132221241561"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "position_id": "4692446793125560154",
                            "code": "A01234",
                            "names": [
                                { "lang": "zh-CN", "value": "高级产品经理" }
                            ],
                            "descriptions": [
                                { "lang": "zh-CN", "value": "岗位的描述" }
                            ],
                            "active": true,
                            "job_family_id_list": ["4719519211875096301"],
                            "job_level_id_list": ["4719519211875096301"],
                            "employee_type_id_list": ["4719519211875096301"],
                            "job_grade_id_list": ["4719519211875096301"],
                            "work_location_id_list": ["4719519211875096301"]
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

        let resp = QueryRequest::new(config)
            .department_ids(vec!["7094136522860922111".to_string()])
            .effective_time("2020-01-01")
            .active(true)
            .fields(vec!["name".to_string()])
            .position_ids(vec!["7094136582860923111".to_string()])
            .position_codes(vec!["42232132221241561".to_string()])
            .page_size(100)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].position_id.as_deref(), Some("4692446793125560154"));
        assert_eq!(
            items[0]
                .names
                .as_ref()
                .and_then(|names| names.first())
                .and_then(|n| n.value.as_deref()),
            Some("高级产品经理")
        );
        assert_eq!(
            items[0]
                .descriptions
                .as_ref()
                .and_then(|descs| descs.first())
                .and_then(|d| d.value.as_deref()),
            Some("岗位的描述")
        );
        assert_eq!(
            items[0].job_grade_id_list.as_ref().map(|l| l.len()),
            Some(1)
        );
        assert_eq!(
            items[0].work_location_id_list.as_ref().map(|l| l.len()),
            Some(1)
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
