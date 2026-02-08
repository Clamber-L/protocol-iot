use std::io;
use std::marker::PhantomData;
use bytes::BytesMut;
use tokio_util::codec::Decoder;
use crate::protocol::traits::{FrameDetector, Protocol, ProtocolParser};

/// 通用编码解析器
pub struct ProtocolCodec<P: Protocol> {
    frame_detector: Box<dyn FrameDetector>,
    parser: Box<dyn ProtocolParser<Message = P::Message>>,
    _phantom_data: PhantomData<P>,
}

impl<P: Protocol> ProtocolCodec<P> {
    pub fn new(protocol: &P) -> Self {
        Self {
            frame_detector: protocol.create_frame_detector(),
            parser: protocol.create_parser(),
            _phantom_data: PhantomData,
        }
    }
}

impl<P: Protocol> Decoder for ProtocolCodec<P> {
    type Item = P::Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.frame_detector.detect_frame(src) {
            Ok(Some(frame_len)) => {
                let frame = src.split_to(frame_len).freeze();
                match self.parser.parse(frame) {
                    Ok(message) => Ok(Some(message)),
                    Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                }
            },
            Ok(None) => Ok(None),
            Err(err) => { Err(io::Error::new(io::ErrorKind::InvalidData, err))  },
        }
    }
}