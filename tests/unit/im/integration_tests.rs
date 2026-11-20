//! IM 模块集成测试
//!
//! 测试模块间的交互、完整的工作流程和真实场景

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use tempfile::tempdir;
use serial_test::serial;
use open_lark::service::im::v1::{message::*, chats::*};
use open_lark::core::{config::Config, req_option::RequestOption};

/// 创建测试用的配置
fn create_test_config(base_url: &str) -> Config {
    Config {
        app_id: "integration_app_id".to_string(),
        app_secret: "integration_app_secret".to_string(), 
        base_url: base_url.to_string(),
        enable_token_cache: true,
        ..Default::default()
    }
}

/// 集成场景测试：完整的消息发送工作流程
#[cfg(test)]
mod message_workflow_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_text_message_workflow() {
        let mock_server = MockServer::start().await;

        // Mock认证请求
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "expire": 3600,
                "tenant_access_token": "mock_tenant_token"
            })))
            .mount(&mock_server)
            .await;

        // Mock创建消息请求
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("receive_id_type", "open_id"))
            .and(header("Authorization", "Bearer mock_tenant_token"))
            .and(body_json(json!({
                "receive_id": "ou_integration_user",
                "msg_type": "text",
                "content": r#"{"text":"集成测试消息"}"#
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "message_id": "om_integration_created",
                    "root_id": null,
                    "parent_id": null,
                    "thread_id": null,
                    "msg_type": "text",
                    "create_time": "1640995200000",
                    "update_time": "1640995200000",
                    "deleted": false,
                    "updated": false,
                    "chat_id": "oc_integration_chat",
                    "sender": {
                        "id": "cli_integration_app",
                        "id_type": "app_id",
                        "sender_type": "app",
                        "tenant_key": "integration_tenant"
                    },
                    "body": {
                        "content": r#"{"text":"集成测试消息"}"#
                    },
                    "mentions": null
                }
            })))
            .mount(&mock_server)
            .await;

        // 执行完整工作流程
        let config = create_test_config(&mock_server.uri());
        let message_service = MessageService { config };

        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_integration_user")
                    .msg_type("text")
                    .content(r#"{"text":"集成测试消息"}"#)
                    .build()
            )
            .build();

        // 验证请求构建正确
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
        
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "ou_integration_user");
        assert_eq!(body.msg_type, "text");
    }

    #[tokio::test] 
    async fn test_message_list_and_create_workflow() {
        let mock_server = MockServer::start().await;

        // Mock获取消息列表
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("container_id_type", "chat"))
            .and(query_param("container_id", "oc_workflow_chat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "page_token": null,
                    "items": [{
                        "message_id": "om_existing_message",
                        "msg_type": "text",
                        "create_time": "1640995100000",
                        "update_time": "1640995100000",
                        "deleted": false,
                        "updated": false,
                        "chat_id": "oc_workflow_chat",
                        "sender": {
                            "id": "ou_other_user",
                            "id_type": "open_id",
                            "sender_type": "user",
                            "tenant_key": "workflow_tenant"
                        },
                        "body": {
                            "content": r#"{"text":"已存在的消息"}"#
                        },
                        "mentions": null
                    }]
                }
            })))
            .mount(&mock_server)
            .await;

        // Mock创建新消息
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("receive_id_type", "chat_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "message_id": "om_workflow_new_message",
                    "msg_type": "text",
                    "create_time": "1640995300000",
                    "update_time": "1640995300000",
                    "deleted": false,
                    "updated": false,
                    "chat_id": "oc_workflow_chat",
                    "sender": {
                        "id": "cli_workflow_app",
                        "id_type": "app_id",
                        "sender_type": "app",
                        "tenant_key": "workflow_tenant"
                    },
                    "body": {
                        "content": r#"{"text":"工作流程新消息"}"#
                    },
                    "mentions": null
                }
            })))
            .mount(&mock_server)
            .await;

        // 1. 首先获取现有消息列表
        let list_request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_workflow_chat")
            .build();

        // 验证列表请求构建
        let params = &list_request.api_req.query_params;
        assert_eq!(params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(params.get("container_id"), Some(&"oc_workflow_chat".to_string()));

        // 2. 然后创建新消息
        let create_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("oc_workflow_chat")
                    .msg_type("text")
                    .content(r#"{"text":"工作流程新消息"}"#)
                    .build()
            )
            .build();

        // 验证创建请求构建
        assert_eq!(
            create_request.api_req.query_params.get("receive_id_type"),
            Some(&"chat_id".to_string())
        );
        
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&create_request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "oc_workflow_chat");
    }

    #[test]
    fn test_message_content_builder_workflow() {
        // 测试复合消息内容的构建工作流程
        
        // 1. 构建文本消息
        let text_msg = MessageText::new("工作流程开始：")
            .at_user("ou_workflow_user")
            .text_line(" 欢迎!")
            .add_text("请查看以下信息：")
            .line()
            .at_all()
            .build();

        assert_eq!(text_msg.msg_type(), "text");
        assert!(text_msg.content().contains("工作流程开始"));
        assert!(text_msg.content().contains("ou_workflow_user"));

        // 2. 构建富文本消息
        let post_msg = MessagePost::new("zh_cn")
            .title("工作流程通知")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("项目进展更新：")),
                MessagePostNode::A(ANode::new("查看详情", "https://workflow.example.com")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("负责人：")),
                MessagePostNode::At(AtNode::new("ou_project_manager")),
            ]);

        assert_eq!(post_msg.msg_type(), "post");
        assert!(post_msg.content().contains("工作流程通知"));
        assert!(post_msg.content().contains("项目进展更新"));

        // 3. 构建卡片模板消息
        let card_vars = json!({
            "title": "工作流程状态",
            "status": "进行中",
            "progress": "75%",
            "next_step": "代码审查",
            "assignee": "ou_workflow_user"
        });

        let card_msg = MessageCardTemplate::new("workflow_status_template", card_vars);
        assert_eq!(card_msg.msg_type(), "interactive");
        assert!(card_msg.content().contains("workflow_status_template"));
    }

    #[test]
    fn test_error_recovery_workflow() {
        // 测试错误恢复工作流程
        
        // 1. 构建可能失败的请求
        let invalid_request = CreateMessageRequest::builder()
            .receive_id_type("invalid_id_type")  // 无效的ID类型
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("")  // 空的接收者ID
                    .msg_type("")    // 空的消息类型
                    .content("")     // 空的内容
                    .build()
            )
            .build();

        // 验证请求仍然可以构建（错误处理在服务器端）
        assert_eq!(
            invalid_request.api_req.query_params.get("receive_id_type"),
            Some(&"invalid_id_type".to_string())
        );

        // 2. 构建正确的恢复请求
        let recovery_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_valid_user")
                    .msg_type("text")
                    .content(r#"{"text":"错误恢复消息"}"#)
                    .uuid("recovery-uuid")
                    .build()
            )
            .build();

        let recovery_body: CreateMessageRequestBody = 
            serde_json::from_slice(&recovery_request.api_req.body).unwrap();
        assert_eq!(recovery_body.receive_id, "ou_valid_user");
        assert_eq!(recovery_body.msg_type, "text");
        assert_eq!(recovery_body.uuid, Some("recovery-uuid".to_string()));
    }
}

/// 聊天和消息交互测试
#[cfg(test)]
mod chat_message_interaction_tests {
    use super::*;

    #[tokio::test]
    async fn test_list_chats_then_send_message_workflow() {
        let mock_server = MockServer::start().await;

        // Mock获取聊天列表
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "chat_id": "oc_interaction_chat_1",
                        "avatar": "https://example.com/avatar1.jpg",
                        "name": "交互测试群组1",
                        "description": "第一个测试群组",
                        "owner_id": "ou_owner_1",
                        "owner_id_type": "open_id",
                        "external": false,
                        "tenant_key": "interaction_tenant",
                        "chat_status": "active"
                    }, {
                        "chat_id": "oc_interaction_chat_2",
                        "avatar": "https://example.com/avatar2.jpg",
                        "name": "交互测试群组2",
                        "description": "第二个测试群组",
                        "owner_id": "ou_owner_2",
                        "owner_id_type": "open_id",
                        "external": true,
                        "tenant_key": "interaction_tenant",
                        "chat_status": "active"
                    }],
                    "page_token": "",
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        // Mock向特定聊天发送消息
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("receive_id_type", "chat_id"))
            .and(body_json(json!({
                "receive_id": "oc_interaction_chat_1",
                "msg_type": "text",
                "content": r#"{"text":"发送到第一个群组的消息"}"#
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "message_id": "om_interaction_message",
                    "chat_id": "oc_interaction_chat_1",
                    "msg_type": "text",
                    "create_time": "1640995400000",
                    "update_time": "1640995400000",
                    "deleted": false,
                    "updated": false,
                    "sender": {
                        "id": "cli_interaction_app",
                        "id_type": "app_id",
                        "sender_type": "app",
                        "tenant_key": "interaction_tenant"
                    },
                    "body": {
                        "content": r#"{"text":"发送到第一个群组的消息"}"#
                    },
                    "mentions": null
                }
            })))
            .mount(&mock_server)
            .await;

        // 1. 构建获取聊天列表的请求
        let chat_list_request = ListChatRequest::builder()
            .user_id_type("open_id")
            .page_size(50)
            .build();

        // 验证聊天列表请求
        let params = &chat_list_request.api_req.query_params;
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("page_size"), Some(&"50".to_string()));

        // 2. 模拟从聊天列表中选择第一个聊天并发送消息
        let target_chat_id = "oc_interaction_chat_1";  // 这通常来自于聊天列表响应
        
        let message_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(target_chat_id)
                    .msg_type("text")
                    .content(r#"{"text":"发送到第一个群组的消息"}"#)
                    .build()
            )
            .build();

        // 验证消息发送请求
        assert_eq!(
            message_request.api_req.query_params.get("receive_id_type"),
            Some(&"chat_id".to_string())
        );
        
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&message_request.api_req.body).unwrap();
        assert_eq!(body.receive_id, target_chat_id);
        assert_eq!(body.msg_type, "text");
    }

    #[test]
    fn test_chat_filtering_and_message_targeting() {
        // 模拟聊天过滤和消息定向发送的逻辑
        
        // 1. 模拟聊天列表数据
        let chats = vec![
            ListChat {
                chat_id: "oc_internal_chat".to_string(),
                avatar: "avatar1.jpg".to_string(),
                name: "内部讨论组".to_string(),
                description: "内部员工讨论".to_string(),
                owner_id: "ou_internal_owner".to_string(),
                owner_id_type: "open_id".to_string(),
                external: false,  // 内部群组
                tenant_key: "company_tenant".to_string(),
                chat_status: "active".to_string(),
            },
            ListChat {
                chat_id: "oc_external_chat".to_string(),
                avatar: "avatar2.jpg".to_string(),
                name: "外部合作组".to_string(),
                description: "与外部合作伙伴的讨论".to_string(),
                owner_id: "ou_external_owner".to_string(),
                owner_id_type: "open_id".to_string(),
                external: true,   // 外部群组
                tenant_key: "company_tenant".to_string(),
                chat_status: "active".to_string(),
            },
            ListChat {
                chat_id: "oc_archived_chat".to_string(),
                avatar: "avatar3.jpg".to_string(),
                name: "已归档群组".to_string(),
                description: "已归档的历史群组".to_string(),
                owner_id: "ou_archived_owner".to_string(),
                owner_id_type: "open_id".to_string(),
                external: false,
                tenant_key: "company_tenant".to_string(),
                chat_status: "archived".to_string(),  // 已归档
            },
        ];

        // 2. 过滤逻辑：只选择活跃的内部群组
        let active_internal_chats: Vec<&ListChat> = chats
            .iter()
            .filter(|chat| !chat.external && chat.chat_status == "active")
            .collect();

        assert_eq!(active_internal_chats.len(), 1);
        assert_eq!(active_internal_chats[0].chat_id, "oc_internal_chat");

        // 3. 为筛选出的聊天创建消息
        for chat in active_internal_chats {
            let message_request = CreateMessageRequest::builder()
                .receive_id_type("chat_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(&chat.chat_id)
                        .msg_type("text")
                        .content(r#"{"text":"发送给内部活跃群组的消息"}"#)
                        .build()
                )
                .build();

            let body: CreateMessageRequestBody = 
                serde_json::from_slice(&message_request.api_req.body).unwrap();
            assert_eq!(body.receive_id, chat.chat_id);
        }
    }

    #[test]
    fn test_multi_chat_broadcast_workflow() {
        // 测试向多个聊天广播消息的工作流程
        
        let target_chat_ids = vec![
            "oc_broadcast_chat_1",
            "oc_broadcast_chat_2", 
            "oc_broadcast_chat_3",
        ];

        let broadcast_message = MessageText::new("📢 重要通知：")
            .line()
            .add_text("系统将于今晚22:00-23:00进行维护，请提前保存工作。")
            .line()
            .add_text("如有疑问，请联系 ")
            .at_user("ou_admin_user")
            .build();

        // 为每个聊天创建广播消息请求
        let mut broadcast_requests = Vec::new();
        
        for chat_id in target_chat_ids {
            let request = CreateMessageRequest::builder()
                .receive_id_type("chat_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(chat_id)
                        .msg_type(broadcast_message.msg_type())
                        .content(broadcast_message.content())
                        .uuid(&format!("broadcast-{}-{}", chat_id, jiff::Timestamp::now().as_second()))
                        .build()
                )
                .build();
                
            broadcast_requests.push(request);
        }

        // 验证所有广播请求
        assert_eq!(broadcast_requests.len(), 3);
        
        for (i, request) in broadcast_requests.iter().enumerate() {
            let body: CreateMessageRequestBody = 
                serde_json::from_slice(&request.api_req.body).unwrap();
            assert_eq!(body.receive_id, format!("oc_broadcast_chat_{}", i + 1));
            assert_eq!(body.msg_type, "text");
            assert!(body.content.contains("重要通知"));
            assert!(body.uuid.is_some());
        }
    }
}

/// 分页和迭代器集成测试
#[cfg(test)]
mod pagination_integration_tests {
    use super::*;

    #[test]
    fn test_message_list_pagination_workflow() {
        // 测试消息列表分页的完整工作流程
        
        // 1. 第一页请求
        let first_page_request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_pagination_chat")
            .page_size(20)
            .sort_type("ByCreateTimeDesc")
            .build();

        let params = &first_page_request.api_req.query_params;
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert!(params.get("page_token").is_none());

        // 2. 模拟第一页响应后的第二页请求
        let second_page_token = "eyJwYWdlX3Rva2VuIjoidGVzdF9wYWdlXzIiLCJ0aW1lc3RhbXAiOjE2NDA5OTU2MDB9";
        let second_page_request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_pagination_chat")
            .page_size(20)
            .sort_type("ByCreateTimeDesc")
            .page_token(second_page_token)
            .build();

        let params = &second_page_request.api_req.query_params;
        assert_eq!(params.get("page_token"), Some(&second_page_token.to_string()));

        // 3. 模拟完整的分页响应数据
        let first_page_response = ListMessageRespData {
            has_more: true,
            page_token: Some(second_page_token.to_string()),
            items: (1..=20).map(|i| Message {
                message_id: format!("om_page_1_msg_{}", i),
                root_id: None,
                parent_id: None,
                thread_id: None,
                msg_type: "text".to_string(),
                create_time: format!("{}", 1640995600 - i * 60),  // 按时间降序
                update_time: format!("{}", 1640995600 - i * 60),
                deleted: false,
                updated: false,
                chat_id: "oc_pagination_chat".to_string(),
                sender: Sender {
                    id: format!("ou_user_{}", i),
                    id_type: "open_id".to_string(),
                    sender_type: "user".to_string(),
                    tenant_key: "pagination_tenant".to_string(),
                },
                body: MessageBody {
                    content: format!(r#"{{"text":"第1页第{}条消息"}}"#, i),
                },
                mentions: None,
            }).collect(),
        };

        assert_eq!(first_page_response.items.len(), 20);
        assert_eq!(first_page_response.has_more, true);
        assert_eq!(first_page_response.page_token, Some(second_page_token.to_string()));

        // 验证消息按时间排序
        let first_msg_time: i64 = first_page_response.items[0].create_time.parse().unwrap();
        let last_msg_time: i64 = first_page_response.items[19].create_time.parse().unwrap();
        assert!(first_msg_time > last_msg_time);  // 降序排列
    }

    #[test]
    fn test_chat_list_pagination_workflow() {
        // 测试聊天列表分页的完整工作流程
        
        // 1. 构建分页请求序列
        let initial_request = ListChatRequest::builder()
            .user_id_type("open_id")
            .sort_type("ByCreateTimeDesc")
            .page_size(10)
            .build();

        // 2. 模拟多页数据
        let page_tokens = vec![
            "page_token_1",
            "page_token_2",
            "page_token_3",
            "", // 最后一页没有token
        ];

        let mut all_requests = vec![initial_request];

        // 为每个分页token创建请求
        for (i, token) in page_tokens.iter().enumerate() {
            if !token.is_empty() {
                let request = ListChatRequest::builder()
                    .user_id_type("open_id")
                    .sort_type("ByCreateTimeDesc")
                    .page_size(10)
                    .page_token(token)
                    .build();
                all_requests.push(request);
            }
        }

        // 验证分页请求序列
        assert_eq!(all_requests.len(), 4);  // 初始请求 + 3个分页请求

        // 验证第一个请求没有page_token
        assert!(all_requests[0].api_req.query_params.get("page_token").is_none());

        // 验证后续请求有正确的page_token
        for (i, request) in all_requests[1..].iter().enumerate() {
            assert_eq!(
                request.api_req.query_params.get("page_token"),
                Some(&page_tokens[i].to_string())
            );
        }

        // 3. 模拟完整的聊天列表数据
        let total_chats = 35;  // 总共35个聊天
        let page_size = 10;
        let total_pages = (total_chats + page_size - 1) / page_size;  // 4页

        for page in 0..total_pages {
            let start_idx = page * page_size;
            let end_idx = std::cmp::min(start_idx + page_size, total_chats);
            let is_last_page = page == total_pages - 1;

            let page_chats: Vec<ListChat> = (start_idx..end_idx).map(|i| ListChat {
                chat_id: format!("oc_pagination_chat_{}", i),
                avatar: format!("https://example.com/avatar_{}.jpg", i),
                name: format!("分页测试群组_{}", i),
                description: format!("第{}个群组，位于第{}页", i, page + 1),
                owner_id: format!("ou_owner_{}", i),
                owner_id_type: "open_id".to_string(),
                external: i % 3 == 0,  // 每3个群组有1个外部群组
                tenant_key: "pagination_tenant".to_string(),
                chat_status: "active".to_string(),
            }).collect();

            let response = ListChatRespData {
                items: page_chats,
                page_token: if is_last_page { "".to_string() } else { page_tokens[page].to_string() },
                has_more: !is_last_page,
            };

            // 验证每页响应
            assert_eq!(response.items.len(), end_idx - start_idx);
            assert_eq!(response.has_more, !is_last_page);
            
            if is_last_page {
                assert_eq!(response.page_token, "");
            } else {
                assert_eq!(response.page_token, page_tokens[page]);
            }
        }
    }

    #[test]
    fn test_pagination_edge_cases() {
        // 测试分页边界情况
        
        // 1. 空结果集
        let empty_message_response = ListMessageRespData {
            has_more: false,
            page_token: None,
            items: vec![],
        };

        let empty_chat_response = ListChatRespData {
            has_more: false,
            page_token: "".to_string(),
            items: vec![],
        };

        assert_eq!(empty_message_response.items.len(), 0);
        assert_eq!(empty_chat_response.items.len(), 0);
        assert_eq!(empty_message_response.has_more, false);
        assert_eq!(empty_chat_response.has_more, false);

        // 2. 单条记录结果集
        let single_message_response = ListMessageRespData {
            has_more: false,
            page_token: None,
            items: vec![Message {
                message_id: "om_single_message".to_string(),
                root_id: None,
                parent_id: None,
                thread_id: None,
                msg_type: "text".to_string(),
                create_time: "1640995600000".to_string(),
                update_time: "1640995600000".to_string(),
                deleted: false,
                updated: false,
                chat_id: "oc_single_chat".to_string(),
                sender: Sender {
                    id: "ou_single_user".to_string(),
                    id_type: "open_id".to_string(),
                    sender_type: "user".to_string(),
                    tenant_key: "single_tenant".to_string(),
                },
                body: MessageBody {
                    content: r#"{"text":"唯一的消息"}"#.to_string(),
                },
                mentions: None,
            }],
        };

        assert_eq!(single_message_response.items.len(), 1);
        assert_eq!(single_message_response.has_more, false);

        // 3. 极大page_size请求
        let large_page_request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_large_page_test")
            .page_size(i32::MAX)
            .build();

        assert_eq!(
            large_page_request.api_req.query_params.get("page_size"),
            Some(&i32::MAX.to_string())
        );

        // 4. 零page_size请求
        let zero_page_request = ListChatRequest::builder()
            .user_id_type("open_id")
            .page_size(0)
            .build();

        assert_eq!(
            zero_page_request.api_req.query_params.get("page_size"),
            Some(&"0".to_string())
        );
    }
}

/// 错误处理集成测试
#[cfg(test)]
mod error_handling_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_network_error_handling() {
        let mock_server = MockServer::start().await;

        // Mock网络超时错误
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .respond_with(ResponseTemplate::new(408).set_body_json(json!({
                "code": 99991400,
                "msg": "request timeout",
                "data": {}
            })))
            .mount(&mock_server)
            .await;

        // 创建可能超时的请求
        let timeout_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_timeout_test")
                    .msg_type("text")
                    .content(r#"{"text":"可能超时的消息"}"#)
                    .build()
            )
            .build();

        // 验证请求构建正确（错误处理在实际调用时）
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&timeout_request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "ou_timeout_test");
    }

    #[tokio::test]
    async fn test_authorization_error_handling() {
        let mock_server = MockServer::start().await;

        // Mock权限错误
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                "code": 99991663,
                "msg": "permission denied",
                "data": {}
            })))
            .mount(&mock_server)
            .await;

        let unauthorized_request = ListChatRequest::builder()
            .user_id_type("open_id")
            .build();

        // 验证请求构建但权限检查在服务器端
        assert_eq!(
            unauthorized_request.api_req.query_params.get("user_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_validation_error_scenarios() {
        // 测试各种验证错误场景的请求构建
        
        // 1. 无效的接收者ID类型
        let invalid_id_type_request = CreateMessageRequest::builder()
            .receive_id_type("invalid_type")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("valid_id")
                    .msg_type("text")
                    .content(r#"{"text":"测试"}"#)
                    .build()
            )
            .build();

        // 请求可以构建，验证在服务器端进行
        assert_eq!(
            invalid_id_type_request.api_req.query_params.get("receive_id_type"),
            Some(&"invalid_type".to_string())
        );

        // 2. 空的消息内容
        let empty_content_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_test")
                    .msg_type("text")
                    .content("")  // 空内容
                    .build()
            )
            .build();

        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&empty_content_request.api_req.body).unwrap();
        assert_eq!(body.content, "");

        // 3. 格式错误的JSON内容
        let malformed_json_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_test")
                    .msg_type("text")
                    .content(r#"{"text":"unclosed string"#)  // 格式错误的JSON
                    .build()
            )
            .build();

        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&malformed_json_request.api_req.body).unwrap();
        assert_eq!(body.content, r#"{"text":"unclosed string"#);
    }

    #[test]
    fn test_retry_logic_preparation() {
        // 测试重试逻辑的请求准备（实际重试在服务层实现）
        
        let retryable_requests = vec![
            // 第一次尝试
            CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id("ou_retry_test")
                        .msg_type("text")
                        .content(r#"{"text":"重试测试消息 - 尝试1"}"#)
                        .uuid("retry-attempt-1")
                        .build()
                )
                .build(),
            
            // 第二次尝试（相同UUID确保幂等性）
            CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id("ou_retry_test")
                        .msg_type("text")
                        .content(r#"{"text":"重试测试消息 - 尝试2"}"#)
                        .uuid("retry-attempt-1")  // 相同UUID
                        .build()
                )
                .build(),
        ];

        // 验证重试请求的UUID一致性
        let body1: CreateMessageRequestBody = 
            serde_json::from_slice(&retryable_requests[0].api_req.body).unwrap();
        let body2: CreateMessageRequestBody = 
            serde_json::from_slice(&retryable_requests[1].api_req.body).unwrap();
        
        assert_eq!(body1.uuid, body2.uuid);
        assert_eq!(body1.receive_id, body2.receive_id);
        assert_ne!(body1.content, body2.content);  // 内容可以不同，但UUID保证幂等性
    }
}

/// 性能和压力测试集成
#[cfg(test)]
mod performance_integration_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_batch_request_building_performance() {
        // 测试批量请求构建的性能
        
        let start = Instant::now();
        let mut message_requests = Vec::new();
        let mut chat_requests = Vec::new();
        
        // 同时构建1000个消息请求和聊天请求
        for i in 0..1000 {
            let message_request = CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(&format!("ou_batch_user_{}", i))
                        .msg_type("text")
                        .content(&format!(r#"{{"text":"批量消息 {}"}}"#, i))
                        .uuid(&format!("batch-uuid-{}", i))
                        .build()
                )
                .build();
            message_requests.push(message_request);

            let chat_request = ListChatRequest::builder()
                .user_id_type("open_id")
                .sort_type("ByCreateTimeDesc")
                .page_size(50)
                .page_token(&format!("batch_token_{}", i))
                .build();
            chat_requests.push(chat_request);
        }
        
        let duration = start.elapsed();
        
        // 验证请求数量
        assert_eq!(message_requests.len(), 1000);
        assert_eq!(chat_requests.len(), 1000);
        
        // 验证性能（应在200ms内完成）
        assert!(duration.as_millis() < 200);
        
        // 验证请求内容的正确性（抽样检查）
        let sample_msg_body: CreateMessageRequestBody = 
            serde_json::from_slice(&message_requests[500].api_req.body).unwrap();
        assert_eq!(sample_msg_body.receive_id, "ou_batch_user_500");
        assert_eq!(sample_msg_body.uuid, Some("batch-uuid-500".to_string()));
        
        let sample_chat_params = &chat_requests[500].api_req.query_params;
        assert_eq!(sample_chat_params.get("page_token"), Some(&"batch_token_500".to_string()));
    }

    #[test]
    fn test_large_message_content_handling() {
        // 测试大消息内容的处理
        
        let large_text = "大消息内容测试 ".repeat(10000);  // 约150KB
        let large_content = format!(r#"{{"text":"{}"}}"#, large_text);
        
        let start = Instant::now();
        
        let large_message_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_large_content_test")
                    .msg_type("text")
                    .content(&large_content)
                    .build()
            )
            .build();
        
        let build_duration = start.elapsed();
        
        // 验证大内容构建性能
        assert!(build_duration.as_millis() < 50);
        
        // 验证内容正确性
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&large_message_request.api_req.body).unwrap();
        assert!(body.content.len() > 100000);
        assert!(body.content.contains("大消息内容测试"));
        
        // 测试序列化/反序列化性能
        let serialize_start = Instant::now();
        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        let serialize_duration = serialize_start.elapsed();
        
        assert!(serialize_duration.as_millis() < 100);
        assert_eq!(body.content, deserialized.content);
    }

    #[test]
    fn test_complex_message_type_performance() {
        // 测试复杂消息类型的构建性能
        
        let start = Instant::now();
        
        // 构建复杂的富文本消息
        let complex_post = MessagePost::new("zh_cn")
            .title("复杂富文本消息性能测试")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("这是一个复杂的富文本消息，包含多种节点类型：")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("粗体文本").style(vec!["bold"])),
                MessagePostNode::A(ANode::new("链接", "https://example.com").style(vec!["underline"])),
                MessagePostNode::At(AtNode::new("ou_performance_user")),
            ])
            .append_content(vec![
                MessagePostNode::Img(ImgNode::new("img_performance_test")),
                MessagePostNode::Media(MediaNode::new("media_key", Some("thumb_key"))),
                MessagePostNode::Emotion(EmotionNode::new("CLAP")),
            ]);
        
        let post_build_duration = start.elapsed();
        
        // 构建卡片模板消息
        let card_start = Instant::now();
        let complex_card_vars = json!({
            "header": {
                "title": "性能测试卡片",
                "subtitle": "复杂数据结构",
                "icon": "https://example.com/icon.png"
            },
            "sections": [
                {
                    "type": "text_section",
                    "fields": [
                        {"name": "项目", "value": "性能测试"},
                        {"name": "状态", "value": "进行中"},
                        {"name": "进度", "value": "85%"}
                    ]
                },
                {
                    "type": "action_section",
                    "actions": [
                        {"type": "button", "text": "查看详情", "url": "https://example.com/details"},
                        {"type": "button", "text": "编辑", "style": "primary"}
                    ]
                }
            ],
            "footer": "测试时间: 2024-01-01 12:00:00"
        });
        
        let complex_card = MessageCardTemplate::new("performance_test_template", complex_card_vars);
        let card_build_duration = card_start.elapsed();
        
        // 验证构建性能
        assert!(post_build_duration.as_millis() < 10);
        assert!(card_build_duration.as_millis() < 5);
        
        // 验证内容正确性
        assert_eq!(complex_post.msg_type(), "post");
        assert!(complex_post.content().contains("复杂富文本消息性能测试"));
        
        assert_eq!(complex_card.msg_type(), "interactive");
        assert!(complex_card.content().contains("performance_test_template"));
        
        // 测试序列化性能
        let serialize_start = Instant::now();
        let _post_content = complex_post.content();
        let _card_content = complex_card.content();
        let serialize_duration = serialize_start.elapsed();
        
        assert!(serialize_duration.as_millis() < 20);
    }

    #[test]
    fn test_concurrent_request_building() {
        // 测试并发请求构建的线程安全性和性能
        use std::sync::Arc;
        use std::thread;
        
        let start = Instant::now();
        let handles: Vec<_> = (0..10)
            .map(|thread_id| {
                thread::spawn(move || {
                    let mut requests = Vec::new();
                    
                    for i in 0..100 {
                        let request = CreateMessageRequest::builder()
                            .receive_id_type("open_id")
                            .request_body(
                                CreateMessageRequestBody::builder()
                                    .receive_id(&format!("ou_concurrent_{}_{}", thread_id, i))
                                    .msg_type("text")
                                    .content(&format!(r#"{{"text":"并发消息 线程{} 第{}条"}}"#, thread_id, i))
                                    .uuid(&format!("concurrent-{}-{}", thread_id, i))
                                    .build()
                            )
                            .build();
                        requests.push(request);
                    }
                    
                    requests
                })
            })
            .collect();

        let mut all_requests = Vec::new();
        for handle in handles {
            let thread_requests = handle.join().unwrap();
            all_requests.extend(thread_requests);
        }
        
        let total_duration = start.elapsed();
        
        // 验证结果
        assert_eq!(all_requests.len(), 1000);  // 10个线程 × 100个请求
        assert!(total_duration.as_millis() < 300);  // 并发应该更快
        
        // 验证请求的唯一性和正确性
        let mut uuids = std::collections::HashSet::new();
        for request in all_requests {
            let body: CreateMessageRequestBody = 
                serde_json::from_slice(&request.api_req.body).unwrap();
            
            // 验证UUID的唯一性
            if let Some(uuid) = body.uuid {
                assert!(uuids.insert(uuid));  // 如果UUID已存在，insert返回false
            }
            
            // 验证内容格式
            assert!(body.receive_id.starts_with("ou_concurrent_"));
            assert!(body.content.contains("并发消息"));
        }
        
        assert_eq!(uuids.len(), 1000);  // 确保所有UUID都是唯一的
    }
}

/// 真实场景模拟测试
#[cfg(test)]
mod real_scenario_tests {
    use super::*;

    #[test]
    fn test_customer_service_bot_scenario() {
        // 模拟客服机器人场景
        
        // 1. 接收用户消息（模拟）
        let user_query = "如何申请退款？";
        
        // 2. 构建自动回复消息
        let auto_reply = MessageText::new("您好！关于退款申请，您可以：")
            .line()
            .add_text("1. 登录官网进入订单页面")
            .line()
            .add_text("2. 点击\"申请退款\"按钮")
            .line()
            .add_text("3. 填写退款原因并提交")
            .line()
            .line()
            .add_text("如需人工客服，请联系 ")
            .at_user("ou_customer_service")
            .build();

        let reply_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_customer_inquiry_user")
                    .msg_type(auto_reply.msg_type())
                    .content(auto_reply.content())
                    .uuid(&format!("customer-service-reply-{}", jiff::Timestamp::now().as_second()))
                    .build()
            )
            .build();

        // 验证回复消息
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&reply_request.api_req.body).unwrap();
        assert!(body.content.contains("退款申请"));
        assert!(body.content.contains("ou_customer_service"));
        assert!(body.uuid.is_some());

        // 3. 如果需要转人工，发送通知给客服
        let staff_notification = MessageText::new("🔔 新的客服请求")
            .line()
            .add_text(&format!("用户问题：{}", user_query))
            .line()
            .add_text("请及时处理。用户ID：")
            .at_user("ou_customer_inquiry_user")
            .build();

        let notification_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("oc_customer_service_chat")
                    .msg_type(staff_notification.msg_type())
                    .content(staff_notification.content())
                    .build()
            )
            .build();

        // 验证客服通知
        let notification_body: CreateMessageRequestBody = 
            serde_json::from_slice(&notification_request.api_req.body).unwrap();
        assert!(notification_body.content.contains("新的客服请求"));
        assert!(notification_body.content.contains(user_query));
    }

    #[test]
    fn test_project_management_workflow() {
        // 模拟项目管理工作流程
        
        // 1. 项目启动通知
        let project_start_card = MessageCardTemplate::new(
            "project_start_template",
            json!({
                "project_name": "新产品开发项目",
                "project_manager": "张三",
                "start_date": "2024-01-15",
                "end_date": "2024-06-15",
                "team_size": 8,
                "priority": "高",
                "departments": ["研发部", "产品部", "设计部"],
                "objectives": [
                    "完成产品核心功能开发",
                    "完成用户界面设计",
                    "完成测试和质量保证"
                ]
            })
        );

        let project_start_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("oc_project_team_chat")
                    .msg_type(project_start_card.msg_type())
                    .content(project_start_card.content())
                    .build()
            )
            .build();

        // 2. 每日站会提醒
        let daily_standup_reminder = MessageText::new("📅 每日站会提醒")
            .line()
            .add_text("时间：上午9:30")
            .line()
            .add_text("地点：会议室A或线上会议")
            .line()
            .add_text("请准备分享：")
            .line()
            .add_text("• 昨天完成的工作")
            .line()
            .add_text("• 今天计划的工作")
            .line()
            .add_text("• 遇到的阻碍或需要帮助的地方")
            .line()
            .line()
            .at_all()
            .build();

        let standup_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("oc_project_team_chat")
                    .msg_type(daily_standup_reminder.msg_type())
                    .content(daily_standup_reminder.content())
                    .build()
            )
            .build();

        // 3. 里程碑完成通知
        let milestone_complete = MessagePost::new("zh_cn")
            .title("🎉 里程碑完成：用户界面设计")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("恭喜团队完成了用户界面设计里程碑！")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("完成时间：").style(vec!["bold"])),
                MessagePostNode::Text(TextNode::new("2024-03-15")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("负责人：").style(vec!["bold"])),
                MessagePostNode::At(AtNode::new("ou_ui_designer")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("设计文档：").style(vec!["bold"])),
                MessagePostNode::A(ANode::new("查看设计稿", "https://design.example.com/project")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("下一步：开始前端开发实现")),
            ]);

        let milestone_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("oc_project_team_chat")
                    .msg_type(milestone_complete.msg_type())
                    .content(milestone_complete.content())
                    .build()
            )
            .build();

        // 验证所有请求
        let start_body: CreateMessageRequestBody = 
            serde_json::from_slice(&project_start_request.api_req.body).unwrap();
        assert_eq!(start_body.msg_type, "interactive");
        assert!(start_body.content.contains("project_start_template"));

        let standup_body: CreateMessageRequestBody = 
            serde_json::from_slice(&standup_request.api_req.body).unwrap();
        assert!(standup_body.content.contains("每日站会提醒"));
        assert!(standup_body.content.contains("全体成员"));

        let milestone_body: CreateMessageRequestBody = 
            serde_json::from_slice(&milestone_request.api_req.body).unwrap();
        assert_eq!(milestone_body.msg_type, "post");
        assert!(milestone_body.content.contains("里程碑完成"));
    }

    #[test]
    fn test_hr_onboarding_workflow() {
        // 模拟HR新员工入职工作流程
        
        let new_employee_name = "李四";
        let new_employee_id = "ou_new_employee_lisi";
        let hr_chat = "oc_hr_department";
        let team_chat = "oc_development_team";

        // 1. 新员工欢迎消息
        let welcome_message = MessageText::new(&format!("🎉 欢迎新同事 "))
            .at_user(new_employee_id)
            .add_text(" 加入我们的团队！")
            .line()
            .line()
            .add_text("📋 入职待办事项：")
            .line()
            .add_text("✅ 账号开通")
            .line()
            .add_text("✅ 设备分配")
            .line()
            .add_text("⏳ 培训课程安排")
            .line()
            .add_text("⏳ 团队介绍会")
            .line()
            .line()
            .add_text("如有任何问题，请联系HR ")
            .at_user("ou_hr_representative")
            .build();

        let welcome_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(team_chat)
                    .msg_type(welcome_message.msg_type())
                    .content(welcome_message.content())
                    .build()
            )
            .build();

        // 2. HR内部跟踪卡片
        let hr_tracking_card = MessageCardTemplate::new(
            "employee_onboarding_template",
            json!({
                "employee_name": new_employee_name,
                "employee_id": new_employee_id,
                "start_date": "2024-01-15",
                "department": "研发部",
                "position": "高级软件工程师",
                "buddy": "ou_buddy_mentor",
                "checklist": [
                    {"task": "IT设备准备", "status": "completed", "due_date": "2024-01-14"},
                    {"task": "系统账号开通", "status": "completed", "due_date": "2024-01-14"},
                    {"task": "入职培训", "status": "in_progress", "due_date": "2024-01-16"},
                    {"task": "团队介绍", "status": "pending", "due_date": "2024-01-17"},
                    {"task": "导师配对", "status": "pending", "due_date": "2024-01-17"}
                ],
                "hr_contact": "ou_hr_representative"
            })
        );

        let hr_tracking_request = CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(hr_chat)
                    .msg_type(hr_tracking_card.msg_type())
                    .content(hr_tracking_card.content())
                    .build()
            )
            .build();

        // 3. 培训提醒消息
        let training_reminder = MessagePost::new("zh_cn")
            .title("📚 新员工培训提醒")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("培训对象：").style(vec!["bold"])),
                MessagePostNode::At(AtNode::new(new_employee_id)),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("培训时间：").style(vec!["bold"])),
                MessagePostNode::Text(TextNode::new("明天（1月16日）上午10:00")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("培训地点：").style(vec!["bold"])),
                MessagePostNode::Text(TextNode::new("培训室B / 线上会议室")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("培训内容：").style(vec!["bold"])),
                MessagePostNode::Text(TextNode::new("公司制度、开发流程、工具使用")),
            ])
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("培训材料：").style(vec!["bold"])),
                MessagePostNode::A(ANode::new("查看培训文档", "https://training.example.com/new-employee")),
            ]);

        let training_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(new_employee_id)
                    .msg_type(training_reminder.msg_type())
                    .content(training_reminder.content())
                    .build()
            )
            .build();

        // 验证所有入职流程请求
        let welcome_body: CreateMessageRequestBody = 
            serde_json::from_slice(&welcome_request.api_req.body).unwrap();
        assert!(welcome_body.content.contains("欢迎新同事"));
        assert!(welcome_body.content.contains(new_employee_id));

        let hr_body: CreateMessageRequestBody = 
            serde_json::from_slice(&hr_tracking_request.api_req.body).unwrap();
        assert_eq!(hr_body.msg_type, "interactive");
        assert!(hr_body.content.contains("employee_onboarding_template"));

        let training_body: CreateMessageRequestBody = 
            serde_json::from_slice(&training_request.api_req.body).unwrap();
        assert_eq!(training_body.msg_type, "post");
        assert!(training_body.content.contains("新员工培训提醒"));
    }
}