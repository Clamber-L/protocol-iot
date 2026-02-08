use bytes::{Bytes, BytesMut};
use std::fmt::Debug;

use crate::error::Result;

pub trait ProtocolMessage: Debug + Send + Sync + Clone {
    // 消息类型标识
    fn message_type(&self) -> &str;

    // 序列化为字节
    fn to_bytes(&self) -> Result<Bytes>;

    // 获取消息摘要
    fn summary(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait FrameDetector: Send + Sync {
    /// 检测帧边界，返回完整帧的长度
    ///
    /// 返回值:
    /// - Ok(Some(len)) - 找到完整帧，长度为len
    /// - Ok(None) - 数据不完整，需要更多数据
    /// - Err(e) - 检测过程中出错
    fn detect_frame(&mut self, src: &BytesMut) -> Result<Option<usize>>;

    // 重置检测器状态
    fn reset(&mut self) {}
}

/// 协议解析器trait
pub trait ProtocolParser: Send + Sync {
    type Message: ProtocolMessage;

    /// 解析字节数据为消息
    fn parse(&self, src: Bytes) -> Result<Self::Message>;

    /// 编码为消息字节
    fn encode(&self, msg: &Self::Message) -> Result<Bytes>;
}

/// 完整的协议定义
pub trait Protocol: Send + Sync + 'static {
    type Message: ProtocolMessage;

    /// 协议名称
    fn name(&self) -> &str;

    /// 创建帧检测器
    fn create_frame_detector(&self) -> Box<dyn FrameDetector>;

    /// 创建解析器
    fn create_parser(&self) -> Box<dyn ProtocolParser<Message = Self::Message>>;

    /// 协议版本
    fn version(&self) -> &str {
        "1.0"
    }
}
