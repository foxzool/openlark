//! 数据帧分包组装（会话内部）。

use std::collections::HashMap;

use lark_websocket_protobuf::pbbp2::Frame;
use log::{debug, error};

use super::headers;

/// 分包消息缓存（按 message_id 聚合）
#[derive(Debug, Default)]
pub(crate) struct FramePackageBuffer {
    sum: usize,
    parts: Vec<Option<Vec<u8>>>,
    received: usize,
}

impl FramePackageBuffer {
    fn new(sum: usize) -> Self {
        Self {
            sum,
            parts: vec![None; sum],
            received: 0,
        }
    }

    fn insert_part(&mut self, seq: usize, payload: Vec<u8>) {
        if seq >= self.sum {
            return;
        }

        if self.parts[seq].is_none() {
            self.received = self.received.saturating_add(1);
        }
        self.parts[seq] = Some(payload);
    }

    fn is_complete(&self) -> bool {
        self.sum > 0 && self.received == self.sum && self.parts.iter().all(|p| p.is_some())
    }

    fn combine(self) -> Vec<u8> {
        let total_len: usize = self
            .parts
            .iter()
            .filter_map(|p| p.as_ref().map(|v| v.len()))
            .sum();
        let mut out = Vec::with_capacity(total_len);
        for part in self.parts.into_iter().flatten() {
            out.extend_from_slice(&part);
        }
        out
    }
}

/// 处理分包：未齐或非法多包（空 `message_id` / `seq` 越界）返回 `None`（不派发）；
/// 齐了返回组合后的帧。
pub(crate) fn assemble_frame(
    buffers: &mut HashMap<String, FramePackageBuffer>,
    mut frame: Frame,
) -> Option<Frame> {
    let hdrs = frame.headers.as_ref();

    let sum: usize = headers::header_usize(hdrs, headers::HDR_SUM).unwrap_or(1);
    let seq: usize = headers::header_usize(hdrs, headers::HDR_SEQ).unwrap_or(0);
    let msg_id: &str = headers::header_value(hdrs, headers::HDR_MESSAGE_ID).unwrap_or("");

    let Some(payload) = frame.payload.take() else {
        error!("Frame payload is empty");
        return None;
    };

    let sum = if sum == 0 { 1 } else { sum };

    if sum == 1 {
        frame.payload = Some(payload);
        return Some(frame);
    }

    // 多包必须可聚合：空 message_id / seq 越界时扣留帧，禁止把残片当完整事件派发（#421 US2）。
    if msg_id.is_empty() {
        error!(
            "multipart frame missing message_id; withhold without dispatch (sum={sum}, seq={seq})"
        );
        return None;
    }

    if seq >= sum {
        error!(
            "multipart frame seq out of range; withhold without dispatch \
             (sum={sum}, seq={seq}, message_id={msg_id})"
        );
        return None;
    }

    let buffer = buffers.entry(msg_id.to_string()).or_insert_with(|| {
        debug!("开始聚合分包消息（sum={sum}, message_id={msg_id}）");
        FramePackageBuffer::new(sum)
    });

    if buffer.sum != sum {
        debug!(
            "分包聚合参数变化，重置缓存（old_sum={}, new_sum={}, message_id={msg_id}）",
            buffer.sum, sum
        );
        *buffer = FramePackageBuffer::new(sum);
    }

    buffer.insert_part(seq, payload);

    if !buffer.is_complete() {
        return None;
    }

    // is_complete 已对 entry 判真，remove 必然命中；勿用空 buffer 兜底掩盖逻辑错误
    let Some(buffer) = buffers.remove(msg_id) else {
        error!("分包缓存丢失（message_id={msg_id}），放弃本帧");
        return None;
    };

    frame.payload = Some(buffer.combine());
    Some(frame)
}
