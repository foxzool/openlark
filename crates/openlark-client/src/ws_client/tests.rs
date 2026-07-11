//! 分包组装单元测试（同步 `package::assemble_frame`）。

use std::collections::HashMap;

use lark_websocket_protobuf::pbbp2::{Frame, Header};

use super::package::{self, FramePackageBuffer};

fn assemble(buffers: &mut HashMap<String, FramePackageBuffer>, frame: Frame) -> Option<Frame> {
    package::assemble_frame(buffers, frame)
}

#[test]
fn test_single_package_payload_preservation() {
    let mut buffers = HashMap::new();
    let test_payload = b"test payload data".to_vec();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_001".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(test_payload.clone()),
        log_id_new: None,
    };

    let result = assemble(&mut buffers, frame);
    assert!(result.is_some());
    let processed_frame = result.unwrap();
    assert_eq!(processed_frame.payload.unwrap(), test_payload);
}

#[test]
fn test_multi_package_payload_combination() {
    let mut buffers = HashMap::new();
    let part1 = b"Hello ".to_vec();
    let part2 = b"World!".to_vec();
    let combined = b"Hello World!".to_vec();

    let frame1 = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_002".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(part1),
        log_id_new: None,
    };

    assert!(assemble(&mut buffers, frame1).is_none());

    let frame2 = Frame {
        seq_id: 2,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_002".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "1".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(part2),
        log_id_new: None,
    };

    let result = assemble(&mut buffers, frame2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().payload.unwrap(), combined);
}

#[test]
fn test_process_frame_packages_missing_payload_returns_none() {
    let mut buffers = HashMap::new();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_missing_payload".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    };

    assert!(assemble(&mut buffers, frame).is_none());
}

#[test]
fn test_process_frame_packages_without_sum_passthrough() {
    let mut buffers = HashMap::new();
    let payload = b"single frame no-sum".to_vec();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_no_sum".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.clone()),
        log_id_new: None,
    };

    let result = assemble(&mut buffers, frame);
    assert!(result.is_some());
    assert_eq!(result.unwrap().payload, Some(payload));
}

#[test]
fn test_process_frame_packages_sum_change_resets_buffer() {
    let mut buffers = HashMap::new();

    let first = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_sum_change".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"A".to_vec()),
        log_id_new: None,
    };

    let second = Frame {
        seq_id: 2,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_sum_change".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "3".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"B".to_vec()),
        log_id_new: None,
    };

    let third = Frame {
        seq_id: 3,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_sum_change".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "3".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "1".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"C".to_vec()),
        log_id_new: None,
    };

    let fourth = Frame {
        seq_id: 4,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "test_msg_sum_change".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "3".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "2".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(b"D".to_vec()),
        log_id_new: None,
    };

    assert!(assemble(&mut buffers, first).is_none());
    assert!(assemble(&mut buffers, second).is_none());
    assert!(assemble(&mut buffers, third).is_none());

    let result = assemble(&mut buffers, fourth);
    assert!(result.is_some());
    assert_eq!(result.unwrap().payload, Some(b"BCD".to_vec()));
}

/// sum>1 但 message_id 为空：无法聚合，降级为单包立即派发（当前行为；与 US2 有张力）。
#[test]
fn assemble_multipart_empty_message_id_degrades_to_single() {
    let mut buffers = HashMap::new();
    let payload = b"orphan-part".to_vec();
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: String::new(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "0".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.clone()),
        log_id_new: None,
    };

    let result = assemble(&mut buffers, frame);
    assert!(result.is_some());
    assert_eq!(result.unwrap().payload, Some(payload));
    assert!(
        buffers.is_empty(),
        "degraded path must not retain package buffer"
    );
}

/// sum>1 但 seq>=sum：越界无法写入缓冲，降级为单包立即派发。
#[test]
fn assemble_multipart_seq_out_of_range_degrades_to_single() {
    let mut buffers = HashMap::new();
    let payload = b"oob-part".to_vec();
    let frame = Frame {
        seq_id: 9,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "msg-oob".to_string(),
            },
            Header {
                key: "sum".to_string(),
                value: "2".to_string(),
            },
            Header {
                key: "seq".to_string(),
                value: "5".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload.clone()),
        log_id_new: None,
    };

    let result = assemble(&mut buffers, frame);
    assert!(result.is_some());
    assert_eq!(result.unwrap().payload, Some(payload));
    assert!(buffers.is_empty());
}
