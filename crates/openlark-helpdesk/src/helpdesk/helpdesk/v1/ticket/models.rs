use serde::{Deserialize, Serialize};

/// 创建工单请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTicketBody {
    /// 标题。
    pub title: String,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// priority 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

/// 创建工单响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTicketResponse {
    /// 工单 ID。
    pub ticket_id: String,
    /// 标题。
    pub title: String,
    /// 创建时间。
    pub created_at: String,
}

/// 获取工单详情响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetTicketResponse {
    /// 工单 ID。
    pub ticket_id: String,
    /// 标题。
    pub title: String,
    /// 描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 状态。
    pub status: String,
    /// 创建时间。
    pub created_at: String,
}

/// 更新工单请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTicketBody {
    /// 标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 标签名列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<String>>,
    /// 评论。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 自定义字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_fields: Option<serde_json::Value>,
    /// 工单类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<i32>,
    /// 是否解决（0=未解决，1=已解决）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solved: Option<i32>,
    /// 渠道。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
}

/// 更新工单响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTicketResponse {
    /// 工单 ID。
    pub ticket_id: String,
    /// 更新时间。
    pub updated_at: String,
}

/// 删除工单响应。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTicketResponse {
    /// 是否成功。
    pub success: bool,
}

/// 工单列表响应。
#[derive(Debug, Clone, Deserialize)]
pub struct TicketListResponse {
    /// 工单列表。
    pub tickets: Vec<TicketItem>,
    /// 下一页分页游标。
    #[serde(default)]
    pub page_token: Option<String>,
}

/// 工单条目。
#[derive(Debug, Clone, Deserialize)]
pub struct TicketItem {
    /// 工单 ID。
    pub ticket_id: String,
    /// 标题。
    pub title: String,
    /// 状态。
    pub status: String,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_ticket_body_serialize() {
        let body = CreateTicketBody {
            title: "无法登录".to_string(),
            description: Some("账号被锁定".to_string()),
            priority: Some("high".to_string()),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(
            value,
            json!({
                "title": "无法登录",
                "description": "账号被锁定",
                "priority": "high"
            })
        );
    }

    #[test]
    fn test_ticket_list_response_deserialize() {
        let value = json!({
            "tickets": [
                {
                    "ticket_id": "t_1",
                    "title": "无法登录",
                    "status": "open"
                }
            ],
            "page_token": "next"
        });

        let resp: TicketListResponse =
            serde_json::from_value(value).expect("deserialize ticket list");
        assert_eq!(resp.tickets.len(), 1);
        assert_eq!(resp.tickets[0].status, "open");
    }
}
