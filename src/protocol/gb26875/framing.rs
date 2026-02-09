use super::types::{END_MARKER, HEADER_SIZE, MAGIC_NUMBER};
use crate::error::{ProtocolError, Result};
use crate::protocol::traits::FrameDetector;
use bytes::BytesMut;

pub struct FrameDetectorImpl {
    max_frame_size: usize,
}

impl FrameDetectorImpl {
    pub fn new() -> Self {
        Self {
            max_frame_size: 2048,
        }
    }
}

impl FrameDetector for FrameDetectorImpl {
    fn detect_frame(&mut self, src: &BytesMut) -> Result<Option<usize>> {
        const MIN_FRAME_SIZE: usize = HEADER_SIZE + 1 + 2;

        if src.len() < MIN_FRAME_SIZE {
            return Ok(None);
        }

        // 查找启动符
        let start_pos = find_magic(src);

        let start_offset = match start_pos {
            Some(start) => start,
            None => {
                // 没找到启动符
                if src.len() >= self.max_frame_size {
                    return Err(ProtocolError::ParseError("No magic number found within max frame size".to_string()));
                }
                return Ok(None);
            }
        };

        if src.len() < start_offset + MIN_FRAME_SIZE {
            return Ok(None);
        }

        // 读取数据长度 - 注意偏移量
        if src.len() < start_offset + HEADER_SIZE {
            return Ok(None);
        }

        let data_len = u16::from_be_bytes([
            src[start_offset + 25], 
            src[start_offset + 26]
        ]) as usize;
        let total_len = HEADER_SIZE + data_len + 1 + 2;

        if total_len > self.max_frame_size {
            return Err(ProtocolError::InvalidLength(total_len));
        }

        if src.len() < start_offset + total_len {
            return Ok(None);
        }

        // 验证结束符
        let end_pos = start_offset + HEADER_SIZE + data_len + 1;
        let end_marker = u16::from_be_bytes([src[end_pos], src[end_pos + 1]]);

        if end_marker != END_MARKER {
            return Err(ProtocolError::ParseError(format!(
                "Invalid end marker: {:#x}",
                end_marker
            )));
        }

        Ok(Some(total_len))
    }
}

fn find_magic(src: &BytesMut) -> Option<usize> {
    let magic = MAGIC_NUMBER.to_be_bytes();
    src.windows(2)
        .position(|w| w[0] == magic[0] && w[1] == magic[1])
}
