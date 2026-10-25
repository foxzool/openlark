//! 获取 OKR 周期列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 周期列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选，默认 50，最大 100）
    page_size: Option<i32>,
    /// 分页标记（可选）
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

    /// 设置分页大小（可选，默认 50，最大 100）
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
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 OKR 周期列表响应数据为空",
        )
        .await
    }
}

/// 获取 OKR 周期列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// OKR 周期列表
    pub items: Vec<OkrPeriod>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// OKR 周期信息（官方 `period`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrPeriod {
    /// 周期 ID
    pub id: String,
    /// 中文名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    /// 英文名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 启用状态
    /// - 0: 正常状态
    /// - 1: 暂不处理
    /// - 2: 标记失效
    /// - 3: 隐藏周期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 周期开始时间（毫秒时间戳字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_start_time: Option<String>,
    /// 周期结束时间（毫秒时间戳字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_end_time: Option<String>,
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
    async fn test_okr_v1_period_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.items[]（id/zh_name/en_name/status/period_*_time）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"items": [{"id": "7071200999834255380", "zh_name": "2022 年 1 月 - 3 月", "en_name": "Jan - Mar 2022", "status": 0, "period_start_time": "1546272000000", "period_end_time": "1577721600000"}], "has_more": false}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/periods"))
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
            .expect("okr_v1_period_list 应成功");

        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "7071200999834255380");
        assert_eq!(
            data.items[0].zh_name.as_deref(),
            Some("2022 年 1 月 - 3 月")
        );
        assert_eq!(data.items[0].en_name.as_deref(), Some("Jan - Mar 2022"));
        assert_eq!(data.items[0].status, Some(0));
        assert_eq!(
            data.items[0].period_start_time.as_deref(),
            Some("1546272000000")
        );
        assert_eq!(
            data.items[0].period_end_time.as_deref(),
            Some("1577721600000")
        );
        assert!(!data.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/periods");
    }
}
