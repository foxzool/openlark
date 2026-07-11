//! 属性测试和健壮性测试模块
//!
//! 这个模块包含了对 open-lark SDK 关键组件的健壮性测试，
//! 使用属性测试来验证代码在各种边界条件下的行为。

pub mod websocket_frame_tests;
pub mod json_parsing_tests;
pub mod http_client_tests;

#[cfg(test)]
mod integration {
    /// 集成属性测试 - 测试多个组件协同工作的健壮性
    
    use super::*;
    use proptest::prelude::*;
    use serde_json::json;

    proptest! {
        /// 端到端属性测试：从 JSON 解析到 WebSocket 帧处理
        #[test]
        fn end_to_end_data_flow_is_robust(
            message_text in "[\\x20-\\x7E]{1,1000}", // 可打印 ASCII
            user_id in "[a-zA-Z0-9_]{10,50}",
            message_id in "[a-zA-Z0-9_]{10,50}",
        ) {
            // 构造有效的消息事件 JSON
            let event_json = json!({
                "ts": "1693834271.787977",
                "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3",
                "token": "v=",
                "type": "event_callback",
                "event": {
                    "sender": {
                        "sender_id": {
                            "open_id": user_id
                        },
                        "sender_type": "user",
                        "tenant_key": "test_tenant"
                    },
                    "message": {
                        "message_id": message_id,
                        "create_time": "1693834271",
                        "chat_id": "oc_test",
                        "chat_type": "group", 
                        "message_type": "text",
                        "content": format!("{{\"text\":\"{}\"}}", message_text),
                        "mentions": []
                    }
                }
            });

            // 序列化为字节
            let payload = serde_json::to_vec(&event_json).unwrap();
            
            // 创建 WebSocket 帧
            let frame = lark_websocket_protobuf::pbbp2::Frame {
                seq_id: 1,
                log_id: 1,
                service: 1,
                method: 1, // 数据帧
                headers: vec![
                    lark_websocket_protobuf::pbbp2::Header {
                        key: "type".to_string(),
                        value: "event".to_string(),
                    },
                    lark_websocket_protobuf::pbbp2::Header {
                        key: "message_id".to_string(),
                        value: message_id.clone(),
                    },
                ],
                payload_encoding: None,
                payload_type: None,
                payload: Some(payload),
                log_id_new: None,
            };

            // 测试帧处理不会崩溃
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let handler = open_lark::event::dispatcher::EventDispatcherHandler::builder().build();
                
                let _result = open_lark::client::ws_client::frame_handler::FrameHandler::handle_frame(frame, &handler).await;
                
                // 不关心具体结果，只要不崩溃即可
            });
        }
    }

    /// 配置解析和网络请求的集成测试
    #[tokio::test]
    async fn test_config_and_http_integration() {
        use std::collections::HashMap;
        
        // 测试各种配置组合
        let configs = vec![
            ("https://api.feishu.cn", "app_123", "secret_456"),
            ("https://open.feishu.cn", "app_xyz", "secret_abc"),
        ];
        
        for (base_url, app_id, app_secret) in configs {
            // 配置应该能被正确创建
            let config = open_lark::core::config::Config::builder(app_id, app_secret)
                .base_url(base_url)
                .build();
            
            assert_eq!(config.app_id, app_id);
            assert_eq!(config.app_secret, app_secret);
            assert_eq!(config.base_url, base_url);
        }
    }
}