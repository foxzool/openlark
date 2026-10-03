//! 飞书/Lark WebSocket 协议的 protobuf 类型。
//!
//! 生成代码随 crate 一起发布，普通构建不需要安装 `protoc`。

/// WebSocket 帧协议类型，由 `protos/pbbp2.proto` 生成。
#[allow(missing_docs)]
pub mod pbbp2;

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::pbbp2::{Frame, Header};

    #[test]
    fn frame_round_trip_preserves_public_message_shape() {
        let frame = Frame {
            seq_id: 7,
            log_id: 11,
            service: 3,
            method: 4,
            headers: vec![Header {
                key: "trace-id".to_owned(),
                value: "trace-001".to_owned(),
            }],
            payload_encoding: Some("none".to_owned()),
            payload_type: Some("application/json".to_owned()),
            payload: Some(br#"{"ok":true}"#.to_vec()),
            log_id_new: Some("log-new".to_owned()),
        };

        let encoded = frame.encode_to_vec();
        let decoded = Frame::decode(encoded.as_slice()).expect("生成的 Frame 应可解码");

        assert_eq!(decoded, frame);
    }
}
