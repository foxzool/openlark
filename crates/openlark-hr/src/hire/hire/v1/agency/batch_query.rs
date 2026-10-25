//! 搜索猎头供应商列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/agency/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// 猎头供应商标签（官方 `agency_supplier_label`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySupplierLabel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商管理员（官方 `agency_supplier_admin`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySupplierAdmin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理员 ID，与入参 `user_id_type` 类型一致。
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理员名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理员邮箱（需 `hire:agency.email:readonly` 权限）。
    pub email: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头简历保护期（官方 `agency_supplier_protect_time`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySupplierProtectTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 保护时长，单位（天）。
    pub day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否使用统一设置。
    pub use_default: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 企业自有简历保护期（官方 `agency_supplier_talent_protect_time`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySupplierTalentProtectTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 保护时长，单位（天）。
    pub day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否使用统一设置。
    pub use_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否永久保护。
    pub forever: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商（官方 `agency_supplier`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySupplier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头供应商 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头供应商名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头标签列表。
    pub label_list: Option<Vec<AgencySupplierLabel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 管理员列表。
    pub admin_list: Option<Vec<AgencySupplierAdmin>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头简历保护期。
    pub agency_protect_time: Option<AgencySupplierProtectTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合作创建时间，毫秒时间戳。
    pub cooperation_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合作开始时间，毫秒时间戳。
    pub cooperation_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合作终止时间，毫秒时间戳。
    pub cooperation_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合作状态：1 正式合作 / 2 试单 / 3 合作终止 / 4 邀请中。
    pub cooperation_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 供应商邮箱。
    pub invite_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头地区：1 中国大陆 / 2 非中国大陆。
    pub supplier_area: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 企业自有简历保护期。
    pub talent_protect_time: Option<AgencySupplierTalentProtectTime>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 搜索猎头供应商列表请求
#[derive(Debug, Clone)]
pub struct BatchQueryRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl BatchQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchQueryResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::AgencyBatchQuery;
        let mut request = ApiRequest::<BatchQueryResponse>::post(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "搜索猎头供应商列表响应数据为空",
        )
        .await
    }
}

/// 搜索猎头供应商列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchQueryResponse {
    #[serde(default, alias = "agencies")]
    /// 结果项列表。
    pub items: Vec<AgencySupplier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for BatchQueryResponse {
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

    /// 端到端：POST /open-apis/hire/v1/agencies/batch_query
    #[tokio::test]
    async fn test_batch_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/agencies/batch_query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "SUCCESS",
                "data": {
                    "has_more": true,
                    "page_token": "eyJvZmZzZXQiOjEsInRpbWVzdGFtcCI6MTY0MDc2NTYzMjA4OCwiaWQiOm51bGx9",
                    "items": [
                        {
                            "id": "7398493486516799788",
                            "name": "北极无敌猎头",
                            "label_list": [
                                {
                                    "id": "6887469228283299336",
                                    "name": { "zh_cn": "东方树叶", "en_us": "oriental Leaves" }
                                }
                            ],
                            "admin_list": [
                                {
                                    "user_id": "7398493486516799788",
                                    "name": { "zh_cn": "张三", "en_us": "ZhangSan" },
                                    "email": "283xxxx2171813@qq.com"
                                }
                            ],
                            "agency_protect_time": { "day": 180, "use_default": true },
                            "cooperation_create_time": "1639992265035",
                            "cooperation_start_time": "1639992265035",
                            "cooperation_end_time": "1639992265035",
                            "cooperation_status": 1,
                            "invite_email": "28933718393.qq.com",
                            "supplier_area": 1,
                            "talent_protect_time": { "day": 180, "use_default": true, "forever": true }
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

        let resp = BatchQueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        let item = resp.items.first().expect("应返回 items 列表");
        assert_eq!(item.id.as_deref(), Some("7398493486516799788"));
        assert_eq!(item.name.as_deref(), Some("北极无敌猎头"));
        let admin = item
            .admin_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("应返回管理员列表");
        assert_eq!(admin.email.as_deref(), Some("283xxxx2171813@qq.com"));
        assert_eq!(
            item.agency_protect_time.as_ref().and_then(|t| t.day),
            Some(180)
        );
        assert_eq!(item.cooperation_status, Some(1));
        assert_eq!(
            item.talent_protect_time.as_ref().and_then(|t| t.forever),
            Some(true)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
