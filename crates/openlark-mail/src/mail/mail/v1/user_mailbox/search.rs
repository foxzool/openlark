//! 搜索邮件
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索邮件请求。
#[derive(Debug, Clone)]
pub struct SearchMailRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: SearchMailRequestBody,
}

/// 搜索邮件请求体。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SearchMailRequestBody {
    /// 搜索关键词（0-50 字符）。
    pub query: Option<String>,
    /// 过滤条件。
    pub filter: Option<MailSearchFilter>,
}

/// 邮件搜索过滤条件。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct MailSearchFilter {
    /// 发件人姓名或邮箱地址筛选。
    pub from: Option<Vec<String>>,
    /// 收件人姓名或邮箱地址筛选。
    pub to: Option<Vec<String>>,
    /// 抄送人姓名或邮箱地址筛选。
    pub cc: Option<Vec<String>>,
    /// 密送人姓名或邮箱地址筛选。
    pub bcc: Option<Vec<String>>,
    /// 邮件主题搜索。
    pub subject: Option<String>,
    /// 文件夹名称筛选（系统文件夹：inbox/sent/drafts 等）。
    pub folder: Option<Vec<String>>,
    /// 自定义标签名称筛选（子标签用 parent_name/child_name）。
    pub label: Option<Vec<String>>,
    /// 是否只筛选有附件的邮件。
    pub has_attachment: Option<bool>,
    /// 是否只筛选未读邮件。
    pub is_unread: Option<bool>,
    /// 邮件接收时间范围筛选。
    pub create_time: Option<MailSearchTimeRange>,
}

/// 邮件搜索时间范围。
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct MailSearchTimeRange {
    /// 开始时间（iso8601，精确到秒）。
    pub start_time: Option<String>,
    /// 截止时间（iso8601，精确到秒）。
    pub end_time: Option<String>,
}

/// 搜索邮件响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchMailResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SearchMailData>,
}

impl ApiResponseTrait for SearchMailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索邮件响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SearchMailData {
    /// 搜索结果列表。
    pub items: Option<Vec<MailSearchItem>>,
    /// 搜索命中结果数。
    pub total: Option<i32>,
    /// 是否还有更多项。
    pub has_more: Option<bool>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 搜索补充提示信息。
    pub notice: Option<String>,
}

/// 邮件搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSearchItem {
    /// 邮件唯一标识。
    pub id: Option<String>,
    /// 包含邮件基本信息的卡片。
    pub display_info: Option<String>,
    /// 邮件元信息。
    pub meta_data: Option<MailSearchMeta>,
}

/// 邮件搜索元信息。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSearchMeta {
    /// 邮件主题。
    pub title: Option<String>,
    /// 邮件线程 ID。
    pub thread_id: Option<String>,
    /// 邮件接收时间。
    pub create_time: Option<String>,
    /// 邮件唯一标识。
    pub message_biz_id: Option<String>,
    /// 邮件发件人。
    pub from: Option<MailAddress>,
}

/// 邮件地址。
#[derive(Debug, Clone, Deserialize)]
pub struct MailAddress {
    /// 邮件地址。
    pub mail_address: Option<String>,
    /// 名称。
    pub name: Option<String>,
}

impl SearchMailRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: SearchMailRequestBody::default(),
        }
    }

    /// 设置搜索关键词。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 设置过滤条件。
    pub fn filter(mut self, filter: MailSearchFilter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchMailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchMailResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/search",
            self.mailbox_id
        );
        let req: ApiRequest<SearchMailResponse> = ApiRequest::post(&path);
        let body = serde_json::to_value(&self.body)
            .map_err(|e| openlark_core::error::validation_error("搜索邮件", format!("请求体序列化失败: {e}")))?;
        let req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("搜索邮件", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_serializes() {
        let filter = MailSearchFilter {
            from: Some(vec!["user@example.com".to_string()]),
            is_unread: Some(true),
            ..Default::default()
        };
        let json = serde_json::to_value(&filter).unwrap();
        assert_eq!(json["from"][0], "user@example.com");
        assert_eq!(json["is_unread"], true);
    }

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0, "msg": "success",
            "data": { "items": [{"id":"msg1","meta_data":{"title":"周会","from":{"mail_address":"a@b.com","name":"A"}}}], "total": 1, "has_more": false }
        }"#;
        let resp: SearchMailResponse = serde_json::from_str(json).unwrap();
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.id, Some("msg1".to_string()));
        assert_eq!(item.meta_data.unwrap().title, Some("周会".to_string()));
    }
}
