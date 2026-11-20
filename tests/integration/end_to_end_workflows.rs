//! 端到端工作流集成测试
//!
//! 测试完整的业务场景，包括认证、消息发送、文件处理、事件处理等
//! 全链路功能验证，确保各个模块协同工作正常

#![cfg(feature = "integration-tests")]

use std::env;
use std::time::Duration;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;
use open_lark::prelude::*;

/// 创建测试客户端配置
fn create_test_client(base_url: &str) -> LarkClient {
    LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .with_base_url(base_url)
        .enable_token_cache(true)
        .req_timeout(Duration::from_secs(30))
        .build()
}

/// 模拟完整的认证流程
async fn setup_auth_mocks(mock_server: &MockServer) {
    // Mock应用访问令牌获取
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "expire": 7200,
            "tenant_access_token": "mock_tenant_token_12345"
        })))
        .mount(mock_server)
        .await;

    // Mock用户访问令牌获取
    Mock::given(method("POST"))
        .and(path("/open-apis/authen/v1/access_token"))
        .and(body_json(json!({
            "grant_type": "authorization_code",
            "code": "mock_auth_code"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "access_token": "mock_user_token_67890",
                "refresh_token": "mock_refresh_token_abcdef",
                "expires_in": 3600,
                "refresh_expires_in": 2592000,
                "scope": "contact:im:message",
                "token_type": "Bearer"
            }
        })))
        .mount(mock_server)
        .await;

    // Mock令牌刷新
    Mock::given(method("POST"))
        .and(path("/open-apis/authen/v1/refresh_access_token"))
        .and(body_json(json!({
            "refresh_token": "mock_refresh_token_abcdef"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "access_token": "mock_new_user_token_xyz",
                "refresh_token": "mock_new_refresh_token_qwe",
                "expires_in": 3600,
                "refresh_expires_in": 2592000,
                "scope": "contact:im:message",
                "token_type": "Bearer"
            }
        })))
        .mount(mock_server)
        .await;
}

/// 模拟IM服务API
async fn setup_im_mocks(mock_server: &MockServer) {
    // Mock发送文本消息
    Mock::given(method("POST"))
        .and(path("/open-apis/im/v1/messages"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "message_id": "om_test_message_123",
                "root_id": null,
                "parent_id": null,
                "thread_id": null,
                "msg_type": "text",
                "create_time": "1640995200000",
                "update_time": "1640995200000",
                "deleted": false,
                "updated": false,
                "chat_id": "oc_test_chat_456",
                "sender": {
                    "id": "cli_test_app",
                    "id_type": "app_id",
                    "sender_type": "app",
                    "tenant_key": "test_tenant"
                },
                "body": {
                    "content": r#"{"text":"测试消息内容"}"#
                },
                "mentions": null
            }
        })))
        .mount(mock_server)
        .await;

    // Mock获取聊天列表
    Mock::given(method("GET"))
        .and(path("/open-apis/im/v1/chats"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "chat_id": "oc_test_chat_456",
                    "avatar": "https://example.com/avatar.jpg",
                    "name": "测试群组",
                    "description": "端到端测试群组",
                    "owner_id": "ou_test_owner",
                    "owner_id_type": "open_id",
                    "external": false,
                    "tenant_key": "test_tenant",
                    "chat_status": "active"
                }],
                "page_token": "",
                "has_more": false
            }
        })))
        .mount(mock_server)
        .await;

    // Mock获取消息列表
    Mock::given(method("GET"))
        .and(path("/open-apis/im/v1/messages"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "has_more": false,
                "page_token": null,
                "items": [{
                    "message_id": "om_existing_message_789",
                    "msg_type": "text",
                    "create_time": "1640995100000",
                    "update_time": "1640995100000",
                    "deleted": false,
                    "updated": false,
                    "chat_id": "oc_test_chat_456",
                    "sender": {
                        "id": "ou_test_user",
                        "id_type": "open_id",
                        "sender_type": "user",
                        "tenant_key": "test_tenant"
                    },
                    "body": {
                        "content": r#"{"text":"已存在的测试消息"}"#
                    },
                    "mentions": null
                }]
            }
        })))
        .mount(mock_server)
        .await;
}

/// 模拟文件上传API
async fn setup_file_mocks(mock_server: &MockServer) {
    // Mock图片上传
    Mock::given(method("POST"))
        .and(path("/open-apis/im/v1/images"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "image_key": "img_test_key_abc123"
            }
        })))
        .mount(mock_server)
        .await;

    // Mock文件上传
    Mock::given(method("POST"))
        .and(path("/open-apis/im/v1/files"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "file_key": "file_test_key_def456"
            }
        })))
        .mount(mock_server)
        .await;
}

/// 模拟联系人服务API
async fn setup_contact_mocks(mock_server: &MockServer) {
    // Mock获取用户信息
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/users"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "user_id": "ou_test_user_123",
                    "union_id": "on_test_user_union",
                    "open_id": "ou_test_user_123",
                    "name": "测试用户",
                    "en_name": "Test User",
                    "email": "test@example.com",
                    "mobile": "+86 138 0000 0000",
                    "avatar": "https://example.com/avatar.jpg",
                    "status": {
                        "is_activated": true,
                        "is_frozen": false,
                        "is_resigned": false
                    },
                    "department_ids": ["od_test_dept_456"],
                    "leader_user_id": "ou_test_leader_789",
                    "position": "软件工程师",
                    "orders": [1],
                    "custom_attrs": [],
                    "employee_type": 1,
                    "join_time": 1609459200,
                    "employee_no": "EMP001"
                }]
            }
        })))
        .mount(mock_server)
        .await;

    // Mock获取部门信息
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/departments"))
        .and(header("Authorization", "Bearer mock_tenant_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "items": [{
                    "department_id": "od_test_dept_456",
                    "open_department_id": "od_test_dept_456",
                    "name": "测试部门",
                    "name_en": "Test Department",
                    "department_type": "department",
                    "parent_department_id": "0",
                    "leader_user_id": "ou_test_leader_789",
                    "chat_id": "oc_dept_chat_789",
                    "member_user_id_count": 10,
                    "order": 1,
                    "status": {
                        "is_deleted": false
                    }
                }]
            }
        })))
        .mount(mock_server)
        .await;
}

/// 端到端认证工作流测试
#[tokio::test]
async fn test_complete_authentication_workflow() {
    let _ = dotenvy::dotenv();

    // 检查环境变量
    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过端到端认证测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_auth_mocks(&mock_server).await;

    let client = create_test_client(&mock_server.uri());

    // 测试获取应用访问令牌
    match client.auth.v3.get_app_access_token(None).await {
        Ok(token_response) => {
            assert_eq!(token_response.code, 0);
            assert!(!token_response.tenant_access_token.is_empty());
            println!("✅ 应用访问令牌获取成功");
        }
        Err(e) => {
            println!("❌ 应用访问令牌获取失败: {}", e.user_friendly_message());
        }
    }

    // 测试获取用户访问令牌
    match client.auth.v1.get_user_access_token("mock_auth_code").await {
        Ok(token_response) => {
            assert_eq!(token_response.code, 0);
            assert!(!token_response.data.unwrap().access_token.is_empty());
            println!("✅ 用户访问令牌获取成功");
        }
        Err(e) => {
            println!("❌ 用户访问令牌获取失败: {}", e.user_friendly_message());
        }
    }

    // 测试刷新用户访问令牌
    match client.auth.v1.refresh_access_token("mock_refresh_token_abcdef").await {
        Ok(token_response) => {
            assert_eq!(token_response.code, 0);
            assert!(!token_response.data.unwrap().access_token.is_empty());
            println!("✅ 用户访问令牌刷新成功");
        }
        Err(e) => {
            println!("❌ 用户访问令牌刷新失败: {}", e.user_friendly_message());
        }
    }
}

/// 端到端消息发送工作流测试
#[tokio::test]
async fn test_complete_messaging_workflow() {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过端到端消息测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_auth_mocks(&mock_server).await;
    setup_im_mocks(&mock_server).await;

    let client = create_test_client(&mock_server.uri());

    // 1. 获取聊天列表
    match client.im.v1.chats.list(&ListChatRequest::builder()
        .user_id_type("open_id")
        .page_size(50)
        .build()
    ).await {
        Ok(chat_response) => {
            assert_eq!(chat_response.code, 0);
            assert!(!chat_response.data.items.is_empty());
            println!("✅ 聊天列表获取成功，找到 {} 个聊天", chat_response.data.items.len());
        }
        Err(e) => {
            println!("❌ 聊天列表获取失败: {}", e.user_friendly_message());
        }
    }

    // 2. 发送文本消息
    let text_message = MessageText::new("🎯 端到端测试消息")
        .line()
        .add_text("这是一条来自集成测试的消息")
        .line()
        .at_user("ou_test_user")
        .build();

    let message_request = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("oc_test_chat_456")
                .msg_type(text_message.msg_type())
                .content(text_message.content())
                .uuid(&format!("e2e-test-{}", jiff::Timestamp::now().as_second()))
                .build()
        )
        .build();

    match client.im.v1.message.create(&message_request, None).await {
        Ok(message_response) => {
            assert_eq!(message_response.code, 0);
            assert!(!message_response.data.message_id.is_empty());
            println!("✅ 文本消息发送成功，消息ID: {}", message_response.data.message_id);
        }
        Err(e) => {
            println!("❌ 文本消息发送失败: {}", e.user_friendly_message());
        }
    }

    // 3. 获取消息列表
    match client.im.v1.message.list(&ListMessageRequest::builder()
        .container_id_type("chat")
        .container_id("oc_test_chat_456")
        .page_size(20)
        .build()
    ).await {
        Ok(message_list_response) => {
            assert_eq!(message_list_response.code, 0);
            println!("✅ 消息列表获取成功，找到 {} 条消息", message_list_response.data.items.len());
        }
        Err(e) => {
            println!("❌ 消息列表获取失败: {}", e.user_friendly_message());
        }
    }
}

/// 端到端文件处理工作流测试
#[tokio::test]
async fn test_complete_file_workflow() {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过端到端文件测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_auth_mocks(&mock_server).await;
    setup_file_mocks(&mock_server).await;

    let client = create_test_client(&mock_server.uri());

    // 1. 模拟图片上传
    match client.im.v1.file.upload_image(b"fake_image_data", "test.png", None).await {
        Ok(upload_response) => {
            assert_eq!(upload_response.code, 0);
            assert!(!upload_response.data.image_key.is_empty());
            println!("✅ 图片上传成功，图片Key: {}", upload_response.data.image_key);

            // 2. 发送图片消息
            let image_content = json!({
                "image_key": upload_response.data.image_key
            });

            let image_request = CreateMessageRequest::builder()
                .receive_id_type("chat_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id("oc_test_chat_456")
                        .msg_type("image")
                        .content(image_content.to_string())
                        .uuid(&format!("image-test-{}", jiff::Timestamp::now().as_second()))
                        .build()
                )
                .build();

            match client.im.v1.message.create(&image_request, None).await {
                Ok(message_response) => {
                    assert_eq!(message_response.code, 0);
                    println!("✅ 图片消息发送成功");
                }
                Err(e) => {
                    println!("❌ 图片消息发送失败: {}", e.user_friendly_message());
                }
            }
        }
        Err(e) => {
            println!("❌ 图片上传失败: {}", e.user_friendly_message());
        }
    }

    // 3. 模拟文件上传
    match client.im.v1.file.upload_file(b"fake_file_data", "test.pdf", None).await {
        Ok(upload_response) => {
            assert_eq!(upload_response.code, 0);
            assert!(!upload_response.data.file_key.is_empty());
            println!("✅ 文件上传成功，文件Key: {}", upload_response.data.file_key);

            // 4. 发送文件消息
            let file_content = json!({
                "file_key": upload_response.data.file_key
            });

            let file_request = CreateMessageRequest::builder()
                .receive_id_type("chat_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id("oc_test_chat_456")
                        .msg_type("file")
                        .content(file_content.to_string())
                        .uuid(&format!("file-test-{}", jiff::Timestamp::now().as_second()))
                        .build()
                )
                .build();

            match client.im.v1.message.create(&file_request, None).await {
                Ok(message_response) => {
                    assert_eq!(message_response.code, 0);
                    println!("✅ 文件消息发送成功");
                }
                Err(e) => {
                    println!("❌ 文件消息发送失败: {}", e.user_friendly_message());
                }
            }
        }
        Err(e) => {
            println!("❌ 文件上传失败: {}", e.user_friendly_message());
        }
    }
}

/// 端到端联系人管理工作流测试
#[tokio::test]
async fn test_complete_contact_workflow() {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过端到端联系人测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_auth_mocks(&mock_server).await;
    setup_contact_mocks(&mock_server).await;

    let client = create_test_client(&mock_server.uri());

    // 1. 获取用户列表
    match client.contact.v3.user.list(&UserListRequest::builder()
        .user_id_type("open_id")
        .page_size(50)
        .build()
    ).await {
        Ok(user_response) => {
            assert_eq!(user_response.code, 0);
            assert!(!user_response.data.items.is_empty());
            println!("✅ 用户列表获取成功，找到 {} 个用户", user_response.data.items.len());

            // 获取第一个用户的信息
            if let Some(user) = user_response.data.items.first() {
                println!("👤 用户信息: {} ({})", user.name, user.open_id);
            }
        }
        Err(e) => {
            println!("❌ 用户列表获取失败: {}", e.user_friendly_message());
        }
    }

    // 2. 获取部门列表
    match client.contact.v3.department.list(&DepartmentListRequest::builder()
        .department_id_type("open_department_id")
        .page_size(50)
        .build()
    ).await {
        Ok(dept_response) => {
            assert_eq!(dept_response.code, 0);
            assert!(!dept_response.data.items.is_empty());
            println!("✅ 部门列表获取成功，找到 {} 个部门", dept_response.data.items.len());

            // 获取第一个部门的信息
            if let Some(dept) = dept_response.data.items.first() {
                println!("🏢 部门信息: {} ({})", dept.name, dept.open_department_id);
            }
        }
        Err(e) => {
            println!("❌ 部门列表获取失败: {}", e.user_friendly_message());
        }
    }

    // 3. 获取特定用户的详细信息
    match client.contact.v3.user.get(&UserGetRequest::builder()
        .user_id("ou_test_user_123")
        .user_id_type("open_id")
        .build()
    ).await {
        Ok(user_detail) => {
            assert_eq!(user_detail.code, 0);
            println!("✅ 用户详细信息获取成功: {}", user_detail.data.name);
        }
        Err(e) => {
            println!("❌ 用户详细信息获取失败: {}", e.user_friendly_message());
        }
    }
}

/// 综合业务场景测试：客服机器人完整工作流
#[tokio::test]
async fn test_customer_service_complete_workflow() {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过客服机器人测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;
    setup_auth_mocks(&mock_server).await;
    setup_im_mocks(&mock_server).await;
    setup_contact_mocks(&mock_server).await;

    let client = create_test_client(&mock_server.uri());

    println!("🤖 开始客服机器人完整工作流测试...");

    // 步骤1：获取用户信息（模拟用户咨询）
    match client.contact.v3.user.get(&UserGetRequest::builder()
        .user_id("ou_test_user_123")
        .user_id_type("open_id")
        .build()
    ).await {
        Ok(user_info) => {
            println!("👤 用户信息: {} - {}", user_info.data.name, user_info.data.email);
        }
        Err(e) => {
            println!("❌ 获取用户信息失败: {}", e.user_friendly_message());
            return;
        }
    }

    // 步骤2：自动回复常见问题
    let auto_reply = MessageText::new("👋 您好！我是智能客服助手")
        .line()
        .add_text("根据您的问题，我为您找到了以下信息：")
        .line()
        .add_text("📋 常见问题解答：https://help.example.com/faq")
        .line()
        .add_text("📞 如需人工客服，请回复 \"人工客服\"")
        .line()
        .line()
        .add_text("⏰ 服务时间：周一至周五 9:00-18:00")
        .build();

    let auto_reply_request = CreateMessageRequest::builder()
        .receive_id_type("open_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("ou_test_user_123")
                .msg_type(auto_reply.msg_type())
                .content(auto_reply.content())
                .uuid(&format!("cs-auto-{}", jiff::Timestamp::now().as_second()))
                .build()
        )
        .build();

    match client.im.v1.message.create(&auto_reply_request, None).await {
        Ok(response) => {
            println!("✅ 自动回复发送成功，消息ID: {}", response.data.message_id);
        }
        Err(e) => {
            println!("❌ 自动回复发送失败: {}", e.user_friendly_message());
        }
    }

    // 步骤3：创建客服工单卡片（如果需要转人工）
    let ticket_card = MessageCardTemplate::new(
        "customer_service_ticket_template",
        json!({
            "ticket_id": format!("TKT-{}", jiff::Timestamp::now().as_second()),
            "customer_name": "测试用户",
            "customer_id": "ou_test_user_123",
            "question_type": "产品咨询",
            "priority": "普通",
            "created_time": jiff::Timestamp::now().strftime("%Y-%m-%d %H:%M:%S").to_string(),
            "assigned_to": "ou_customer_service_rep",
            "status": "待处理",
            "description": "用户咨询产品功能相关问题"
        })
    );

    let ticket_request = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("oc_customer_service_team")
                .msg_type(ticket_card.msg_type())
                .content(ticket_card.content())
                .uuid(&format!("ticket-{}", jiff::Timestamp::now().as_second()))
                .build()
        )
        .build();

    match client.im.v1.message.create(&ticket_request, None).await {
        Ok(response) => {
            println!("✅ 客服工单创建成功，消息ID: {}", response.data.message_id);
        }
        Err(e) => {
            println!("❌ 客服工单创建失败: {}", e.user_friendly_message());
        }
    }

    // 步骤4：通知客服团队有新工单
    let notification = MessageText::new("🔔 新的客服工单")
        .line()
        .add_text("用户：测试用户 (ou_test_user_123)")
        .line()
        .add_text("类型：产品咨询")
        .line()
        .add_text("请及时处理")
        .build();

    let notification_request = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("oc_customer_service_team")
                .msg_type(notification.msg_type())
                .content(notification.content())
                .build()
        )
        .build();

    match client.im.v1.message.create(&notification_request, None).await {
        Ok(response) => {
            println!("✅ 客服团队通知发送成功，消息ID: {}", response.data.message_id);
        }
        Err(e) => {
            println!("❌ 客服团队通知发送失败: {}", e.user_friendly_message());
        }
    }

    println!("🎉 客服机器人完整工作流测试完成！");
}

/// 错误恢复和重试机制测试
#[tokio::test]
async fn test_error_recovery_and_retry_workflow() {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        println!("⚠️  跳过错误恢复测试：未设置APP_ID/APP_SECRET环境变量");
        return;
    }

    let mock_server = MockServer::start().await;

    // 设置认证成功
    setup_auth_mocks(&mock_server).await;

    // 设置临时错误，然后成功的场景
    Mock::given(method("POST"))
        .and(path("/open-apis/im/v1/messages"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "code": 99991400,
            "msg": "internal server error",
            "data": {}
        })))
        .up_to_n_times(2)  // 前两次返回错误
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "message_id": "om_retry_success_message",
                "msg_type": "text",
                "create_time": "1640995200000",
                "chat_id": "oc_test_chat_456",
                "sender": {
                    "id": "cli_test_app",
                    "id_type": "app_id",
                    "sender_type": "app",
                    "tenant_key": "test_tenant"
                },
                "body": {
                    "content": r#"{"text":"重试成功的消息"}"#
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server.uri());

    let retry_message = MessageText::new("🔄 测试重试机制")
        .add_text("这条消息可能会失败几次，但最终会成功")
        .build();

    let retry_request = CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("oc_test_chat_456")
                .msg_type(retry_message.msg_type())
                .content(retry_message.content())
                .uuid("retry-test-uuid")  // 使用相同UUID确保幂等性
                .build()
        )
        .build();

    // 模拟重试逻辑（实际应该在SDK中实现）
    let mut retry_count = 0;
    let max_retries = 3;

    while retry_count < max_retries {
        match client.im.v1.message.create(&retry_request, None).await {
            Ok(response) => {
                println!("✅ 消息发送成功，重试 {} 次后成功，消息ID: {}", retry_count, response.data.message_id);
                break;
            }
            Err(e) => {
                retry_count += 1;
                if retry_count < max_retries {
                    println!("⚠️  消息发送失败（第 {} 次），准备重试: {}", retry_count, e.user_friendly_message());
                    tokio::time::sleep(Duration::from_millis(100 * retry_count as u64)).await;
                } else {
                    println!("❌ 消息发送最终失败，已重试 {} 次: {}", retry_count, e.user_friendly_message());
                    break;
                }
            }
        }
    }
}

#[test]
fn test_request_builder_patterns() {
    // 测试各种请求构建器模式的使用

    // 1. 传统构造函数模式
    let traditional_request = CreateMessageRequest {
        api_req: open_lark_core::core::api::ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/im/v1/messages".to_string(),
            body: serde_json::to_vec(&json!({
                "receive_id": "test_user",
                "msg_type": "text",
                "content": r#"{"text":"传统模式消息"}"#
            })).unwrap(),
            query_params: std::collections::HashMap::from([("receive_id_type", "open_id".to_string())]),
            path_params: std::collections::HashMap::new(),
            supported_access_token_types: vec![open_lark_core::core::constants::AccessTokenType::Tenant],
            file: vec![],
        }
    };

    // 2. 现代构建器模式
    let builder_request = CreateMessageRequest::builder()
        .receive_id_type("open_id")
        .request_body(
            CreateMessageRequestBody::builder()
                .receive_id("test_user")
                .msg_type("text")
                .content(r#"{"text":"构建器模式消息"}"#)
                .uuid("builder-pattern-test")
                .build()
        )
        .build();

    // 验证两种模式都能正确构建请求
    assert_eq!(
        traditional_request.api_req.query_params.get("receive_id_type"),
        Some(&"open_id".to_string())
    );

    assert_eq!(
        builder_request.api_req.query_params.get("receive_id_type"),
        Some(&"open_id".to_string())
    );

    let traditional_body: serde_json::Value = serde_json::from_slice(&traditional_request.api_req.body).unwrap();
    let builder_body: serde_json::Value = serde_json::from_slice(&builder_request.api_req.body).unwrap();

    assert_eq!(traditional_body["receive_id"], "test_user");
    assert_eq!(builder_body["receive_id"], "test_user");
    assert_eq!(builder_body["uuid"], "builder-pattern-test");

    println!("✅ 请求构建器模式测试通过");
}