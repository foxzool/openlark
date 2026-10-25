//! 获取假期类型列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/leave_types>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取假期类型列表请求
#[derive(Debug, Clone)]
pub struct LeaveTypesRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl LeaveTypesRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<LeaveTypesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LeaveTypesResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveLeaveTypes;
        let mut request = ApiRequest::<LeaveTypesResponse>::get(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取假期类型列表响应数据为空",
        )
        .await
    }
}

/// 获取假期类型列表响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveTypesResponse {
    /// 假期类型列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_list: Option<Vec<LeaveTypeItem>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 假期类型条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveTypeItem {
    /// 假期类型ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 假期类型名称（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_name: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 假期类型状态（1 已启用 / 2 已停用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 假期子类列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_subtype_list: Option<Vec<LeaveSubtype>>,
    /// 假期类型的创建时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 假期类型的创建人，值为创建人的员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 假期类型的更新时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 假期类型的更新人，值为更新人的员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 假期子类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveSubtype {
    /// 假期子类ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 假期子类名称（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_name: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for LeaveTypesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v1/leaves/leave_types
    #[tokio::test]
    async fn test_leave_types_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/leaves/leave_types"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "leave_type_list": [
                        {
                            "leave_type_id": "4718803945687580505",
                            "leave_type_name": [{"lang": "zh_CN", "value": "张三"}],
                            "status": 2,
                            "leave_subtype_list": [
                                {
                                    "leave_type_id": "4718803945687580505",
                                    "leave_type_name": [{"lang": "zh_CN", "value": "张三"}]
                                }
                            ],
                            "created_at": "1658730384",
                            "created_by": "7104164629974646786",
                            "updated_at": "1658996995",
                            "updated_by": "7112112899636854812"
                        }
                    ],
                    "has_more": true,
                    "page_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
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

        let resp = LeaveTypesRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let list = resp
            .leave_type_list
            .expect("leave_type_list 应解析出假期类型列表");
        assert_eq!(list.len(), 1);
        assert_eq!(
            list[0].leave_type_id.as_deref(),
            Some("4718803945687580505")
        );
        assert_eq!(list[0].status, Some(2));
        assert_eq!(
            list[0]
                .leave_type_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );
        assert_eq!(
            list[0].leave_subtype_list.as_ref().map(|s| s.len()),
            Some(1)
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
