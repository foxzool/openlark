//! 批量查询员工薪资档案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/archive/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量查询员工薪资档案请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
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
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::ArchiveQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询员工薪资档案响应数据为空",
        )
        .await
    }
}

/// 批量查询员工薪资档案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 薪资档案列表
    pub items: Vec<Archive>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 薪资档案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Archive {
    /// 员工薪资档案 ID
    pub id: String,
    /// 员工 ID，类型与查询参数 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 员工薪资档案版本 ID，薪资档案唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    /// 关联薪资方案 ID（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// 关联薪资方案版本 ID，薪资方案唯一标识（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_tid: Option<String>,
    /// 档案币种 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 调薪原因 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_reason_id: Option<String>,
    /// 调薪说明（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_description: Option<String>,
    /// 生效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 失效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 薪级薪等 ID（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary_level_id: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 档案关联的薪资项（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_items: Option<Vec<ArchiveItem>>,
    /// 档案关联的薪资指标（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_indicators: Option<Vec<ArchiveIndicator>>,
}

/// 档案关联的薪资项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveItem {
    /// 薪资项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 档案关联薪酬项数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_result: Option<String>,
    /// 档案关联薪酬项转正后数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_result_regular: Option<String>,
}

/// 档案关联的薪资指标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveIndicator {
    /// 薪资统计指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 档案关联薪资统计指标数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_result: Option<String>,
    /// 档案关联薪资统计指标转正后数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_result_regular: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
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
    async fn test_query_archives_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.items[]（含 archive_items/archive_indicators）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"items": [{"user_id": "344325234123", "id": "32141234412", "tid": "141541351", "plan_id": "123412433", "plan_tid": "213412343", "currency_id": "324167823", "change_reason_id": "321414123", "change_description": "长期表现优异，产出高，本次给出涨幅", "effective_date": "2022-10-23", "expiration_date": "2022-10-24", "salary_level_id": "12342313", "created_time": "2024-01-01 00:00:00", "updated_time": "2024-01-01 00:00:00", "archive_items": [{"item_id": "213423144", "item_result": "15000", "item_result_regular": "18000"}], "archive_indicators": [{"indicator_id": "23141423", "indicator_result": "15000", "indicator_result_regular": "18000"}]}], "page_token": "1234452132", "has_more": true}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives/query"))
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

        let data = QueryRequest::new(config)
            .execute()
            .await
            .expect("查询薪资档案应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "32141234412");
        assert_eq!(data.items[0].tid.as_deref(), Some("141541351"));
        assert_eq!(data.items[0].change_reason_id.as_deref(), Some("321414123"));
        assert_eq!(
            data.items[0].archive_items.as_ref().unwrap()[0]
                .item_id
                .as_deref(),
            Some("213423144")
        );
        assert_eq!(
            data.items[0].archive_indicators.as_ref().unwrap()[0]
                .indicator_result
                .as_deref(),
            Some("15000")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/archives/query"
        );
    }
}
