mod framing;
mod parser;
mod types;

use crate::protocol::{
    gb26875::{parser::ParserImpl, types::ProtocolAMessage},
    traits::*,
};
use framing::FrameDetectorImpl;

/// Protocol A - 你当前的协议
pub struct ProtocolA;

impl Protocol for ProtocolA {
    type Message = ProtocolAMessage;

    fn name(&self) -> &str {
        "ProtocolA"
    }

    fn create_frame_detector(&self) -> Box<dyn FrameDetector> {
        Box::new(FrameDetectorImpl::new())
    }

    fn create_parser(&self) -> Box<dyn ProtocolParser<Message = Self::Message>> {
        Box::new(ParserImpl)
    }

    fn version(&self) -> &str {
        "1.0"
    }
}
