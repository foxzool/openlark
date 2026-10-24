//! 获取薪资组基本信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/paygroup/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取薪资组基本信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选，默认 100，最大 2000）
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

    /// 设置分页大小（可选，默认 100，最大 2000）
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

        // 1. 校验分页参数范围
        if let Some(page_size) = self.page_size
            && !(1..=2000).contains(&page_size)
        {
            return Err(validation_error(
                "page_size 超出范围",
                "分页大小取值范围为 1~2000",
            ));
        }

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::PaygroupList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取薪资组基本信息响应数据为空",
        )
        .await
    }
}

/// 获取薪资组基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 薪资组列表
    pub items: Vec<Paygroup>,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

/// 薪资组信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Paygroup {
    /// 薪资组 ID
    pub pay_group_id: String,
    /// 薪资组名称（多语种）
    pub name: PaygroupName,
    /// 薪资组编码
    pub code: String,
    /// 薪资组状态：1-启用 0-禁用
    pub status: i32,
}

/// 薪资组名称（多语种）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaygroupName {
    /// 中文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
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
    async fn test_list_paygroups_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [{
                    "pay_group_id": "6823630319749580301",
                    "name": {"zh_cn": "CHN-审核", "en_us": "english name"},
                    "code": "PAY_GROUP-3-8",
                    "status": 1
                }],
                "page_token": "6862995757234914824",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/paygroups"))
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
            .expect("查询薪资组应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].pay_group_id, "6823630319749580301");
        assert_eq!(data.items[0].name.zh_cn, Some("CHN-审核".to_string()));
        assert_eq!(data.items[0].name.en_us, Some("english name".to_string()));
        assert_eq!(data.items[0].code, "PAY_GROUP-3-8");
        assert_eq!(data.items[0].status, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/payroll/v1/paygroups");
    }
}
