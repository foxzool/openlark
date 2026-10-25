//! 获取外部数据源配置信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/datasource/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取外部数据源配置信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填，表示从头开始遍历）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（必填，取值范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
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
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段
        let page_size = self.page_size.ok_or_else(|| {
            validation_error("page_size 不能为空", "分页大小必填，取值范围 1~100")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(validation_error(
                "page_size 超出范围",
                "分页大小取值范围为 1~100",
            ));
        }

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::DatasourceList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取外部数据源配置信息响应数据为空",
        )
        .await
    }
}

/// 获取外部数据源配置信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 数据源列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasources: Option<Vec<DatasourceItem>>,
}

/// 外部数据源配置（官方 datasource）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceItem {
    /// 数据源编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 数据源名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
    /// 启停用状态：1-已启用 2-已停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    /// 数据源字段列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DatasourceField>>,
    /// 数据源描述（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    /// 数据写入维度：1-算薪期间 2-数据发生日期 3-自定义数据周期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_period_type: Option<i32>,
}

/// 数据源字段（官方 datasource_field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasourceField {
    /// 数据源字段编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 数据源字段名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
    /// 字段类型：1-金额 2-数值 3-文本 4-日期 5-百分比
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
    /// 字段启停用状态：1-已启用 2-已停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    /// 数据源字段描述（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    /// 保留小数位数（目前只有 number、money 类型字段需要设置）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
}

/// 多语种内容（官方 i18n_content，id 为名称对应的实体 id）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语种，如 zh_cn、en_us
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 语种对应的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 名称对应的实体 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    async fn test_list_datasources_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data 为 page_token/has_more/datasources[]
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                "has_more": true,
                "datasources": [
                    {
                        "code": "test_datasource__c",
                        "i18n_names": [
                            {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                        ],
                        "active_status": 1,
                        "fields": [
                            {
                                "code": "test__c",
                                "i18n_names": [
                                    {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                                ],
                                "field_type": 1,
                                "active_status": 1,
                                "i18n_description": [
                                    {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                                ],
                                "decimal_places": 1
                            }
                        ],
                        "i18n_description": [
                            {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                        ],
                        "data_period_type": 1
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/datasources"))
            .and(query_param("page_size", "10"))
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
            .page_size(10)
            .execute()
            .await
            .expect("获取外部数据源配置信息应成功");

        assert!(data.has_more);
        let datasources = data.datasources.as_ref().expect("datasources 应反序列化");
        assert_eq!(datasources.len(), 1);
        assert_eq!(datasources[0].code.as_deref(), Some("test_datasource__c"));
        assert_eq!(datasources[0].active_status, Some(1));
        assert_eq!(datasources[0].data_period_type, Some(1));
        assert_eq!(
            datasources[0].i18n_names.as_ref().unwrap()[0].id.as_deref(),
            Some("723123123123123213")
        );
        let field = &datasources[0].fields.as_ref().unwrap()[0];
        assert_eq!(field.code.as_deref(), Some("test__c"));
        assert_eq!(field.field_type, Some(1));
        assert_eq!(field.decimal_places, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/payroll/v1/datasources");
    }
}
