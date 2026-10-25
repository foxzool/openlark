//! 查询指定时间范围地点版本
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/organization-management/location/query_multi_timeline>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询指定时间范围地点版本请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineBody {
    /// 地点 ID 列表（必填，长度范围 1～10）
    pub location_ids: Vec<String>,
    /// 查询开始时间（包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 查询结束时间（不包含），格式 "YYYY-MM-DD"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 返回数据的字段列表，可选 location_name/code/active 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

/// 查询指定时间范围地点版本的响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryMultiTimelineResponse {
    /// 地点信息列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LocationTimelineItem>>,
}

/// 地点时间轴信息（官方 location_timeline 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationTimelineItem {
    /// 地点版本信息。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_version_data: Option<Vec<LocationVersionData>>,
    /// 地址信息。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<LocationAddress>>,
}

/// 地点版本信息（官方 location_version_data 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationVersionData {
    /// 地点 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// 地点版本 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_version_id: Option<String>,
    /// 地点名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_names: Option<Vec<I18nText>>,
    /// 上级地点 ID（若查询的是一级地点，则该字段不展示）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_location_id: Option<String>,
    /// 当前版本生效日期（YYYY-MM-DD）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 当前版本失效日期（YYYY-MM-DD）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 是否启用。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 描述。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 编码。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 地点用途。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_usages: Option<Vec<EnumValue>>,
    /// 区域设置 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<EnumValue>,
    /// 时区 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    /// 默认语言 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
    /// 工时制度 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
}

/// 地址信息（官方 address 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationAddress {
    /// 完整地址（本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_local_script: Option<String>,
    /// 完整地址（西方文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_address_western_script: Option<String>,
    /// 地址 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// 国家 / 地区 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 主要行政区 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 城市 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city_id: Option<String>,
    /// 区县 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_id: Option<String>,
    /// 地址行 1。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// 地址行 2。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    /// 地址行 3。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    /// 地址行 4。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line4: Option<String>,
    /// 地址行 5。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line5: Option<String>,
    /// 地址行 6。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line6: Option<String>,
    /// 地址行 7。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line7: Option<String>,
    /// 地址行 8。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line8: Option<String>,
    /// 地址行 9。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line9: Option<String>,
    /// 地址行 1（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line1: Option<String>,
    /// 地址行 2（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line2: Option<String>,
    /// 地址行 3（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line3: Option<String>,
    /// 地址行 4（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line4: Option<String>,
    /// 地址行 5（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line5: Option<String>,
    /// 地址行 6（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line6: Option<String>,
    /// 地址行 7（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line7: Option<String>,
    /// 地址行 8（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line8: Option<String>,
    /// 地址行 9（非拉丁语系的本地文字）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_address_line9: Option<String>,
    /// 邮政编码。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// 地址类型。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_type_list: Option<Vec<EnumValue>>,
    /// 主要地址。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 公开地址。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct I18nText {
    /// 语言（中文用 zh-CN，英文用 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}
impl ApiResponseTrait for QueryMultiTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 查询指定时间范围地点版本的请求。
#[derive(Debug, Clone)]
pub struct QueryMultiTimelineRequest {
    config: Arc<Config>,
    body: QueryMultiTimelineBody,
}
/// 查询指定时间范围地点版本请求构建器实现。
impl QueryMultiTimelineRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryMultiTimelineBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: QueryMultiTimelineBody) -> Self {
        self.body = body;
        self
    }
    /// 设置地点 ID 列表（必填，长度范围 1～10）。
    pub fn location_ids(mut self, location_ids: Vec<String>) -> Self {
        self.body.location_ids = location_ids;
        self
    }
    /// 设置查询开始时间（包含），格式 "YYYY-MM-DD"。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.body.start_date = Some(start_date.into());
        self
    }
    /// 设置查询结束时间（不包含），格式 "YYYY-MM-DD"。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = Some(end_date.into());
        self
    }
    /// 设置返回数据的字段列表。
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.body.fields = Some(fields);
        self
    }
    /// 执行查询指定时间范围地点版本请求。
    pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryMultiTimelineResponse> {
        validate_required!(self.body.location_ids, "location_ids 不能为空且为 1～10 个");
        if self.body.location_ids.len() > 10 {
            return Err(CoreError::validation_msg("location_ids 长度范围为 1～10"));
        }

        let request = ApiRequest::<QueryMultiTimelineResponse>::post(
            "/open-apis/corehr/v2/locations/query_multi_timeline",
        )
        .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询指定时间范围地点版本",
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

    /// 端到端：POST /open-apis/corehr/v2/locations/query_multi_timeline
    #[tokio::test]
    async fn test_query_multi_timeline_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/locations/query_multi_timeline"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "location_version_data": [
                                {
                                    "location_id": "4719456877659520852",
                                    "location_version_id": "7238516215202170412",
                                    "location_names": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "parent_location_id": "8961456877659520953",
                                    "effective_date": "2020-05-01",
                                    "expiration_date": "2020-05-02",
                                    "active": true,
                                    "descriptions": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ],
                                    "code": "BD38591"
                                }
                            ],
                            "address": [
                                {
                                    "full_address_local_script": "中国北京北京",
                                    "full_address_western_script": "Beijing, Beijing, China,",
                                    "address_id": "6989822217869624863",
                                    "country_region_id": "6862995757234914824",
                                    "address_line1": "丹佛测试地址-纽埃时区",
                                    "postal_code": "611530",
                                    "is_primary": true,
                                    "is_public": true
                                }
                            ]
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

        let resp = QueryMultiTimelineRequest::new(Arc::new(config))
            .location_ids(vec!["4719456877659520852".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        let version_data = items[0]
            .location_version_data
            .clone()
            .expect("location_version_data 应存在");
        assert_eq!(version_data.len(), 1);
        assert_eq!(
            version_data[0].location_version_id,
            Some("7238516215202170412".to_string())
        );
        assert_eq!(
            version_data[0].parent_location_id,
            Some("8961456877659520953".to_string())
        );
        assert_eq!(
            version_data[0].location_names,
            Some(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
        );
        let address = items[0].address.clone().expect("address 应存在");
        assert_eq!(address.len(), 1);
        assert_eq!(address[0].is_primary, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
