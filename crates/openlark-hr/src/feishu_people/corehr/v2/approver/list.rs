//! 获取指定人员审批任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/approver/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取指定人员审批任务列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（必填，取值范围 1～100，默认 20）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
    /// 指定人员 id（必填，按 user_id_type 类型传递）
    user_id: String,
    /// 任务状态（-1 发起 / 1 进行中 / 2 已拒绝 / 3 已通过 / 5 抄送 / 6 表单提交 / 12 失败 / 14 已回退）
    approver_status: Option<i32>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            user_id: String::new(),
            approver_status: None,
        }
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

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置指定人员 id（必填，按 user_id_type 类型传递）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置任务状态。
    pub fn approver_status(mut self, approver_status: i32) -> Self {
        self.approver_status = Some(approver_status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }
        validate_required!(self.user_id, "user_id 不能为空");

        let request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/approvers")
            .query("page_size", page_size.to_string())
            .query("user_id", self.user_id)
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt(
                "approver_status",
                self.approver_status.map(|status| status.to_string()),
            );

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取指定人员审批任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 审批任务列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_list: Option<Vec<ApproverItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 审批人条目信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApproverItem {
    /// 审批任务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 流程 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 任务状态：-2 跳过 / -1 发起 / 0 未开始 / 1 进行中 / 2 已拒绝 / 3 已通过 / 4 被撤回 / 5 抄送 / 6 表单提交 / 12 失败 / 14 已回退 / 16 发起撤销 / 18 已干预
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_status: Option<i32>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/approvers
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/approvers"))
            .and(query_param("page_size", "20"))
            .and(query_param("user_id", "ou_91791271921729102012"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "page_token": "1",
                    "has_more": true,
                    "approver_list": [
                        {
                            "approver_id": "7410781046418966060",
                            "process_id": "7410781046418966060",
                            "approver_status": 1
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

        let resp = ListRequest::new(config)
            .page_size(20)
            .user_id("ou_91791271921729102012")
            .execute()
            .await
            .expect("请求应成功");

        let list = resp.approver_list.expect("应解析审批任务列表");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].approver_id, Some("7410781046418966060".to_string()));
        assert_eq!(list[0].process_id, Some("7410781046418966060".to_string()));
        assert_eq!(list[0].approver_status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
