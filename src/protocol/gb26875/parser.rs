use super::types::*;
use crate::error::{ProtocolError, Result};
use crate::protocol::traits::ProtocolParser;
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct ParserImpl;

impl ProtocolParser for ParserImpl {
    type Message = ProtocolAMessage;

    fn parse(&self, mut data: Bytes) -> Result<Self::Message> {
        let header = parse_header(&mut data)?;

        let data_len = header.data_unit_len as usize;
        let app_data = data.copy_to_bytes(data_len);

        let checksum = data.get_u8();
        let _end_marker = data.get_u16();

        Ok(ProtocolAMessage {
            header,
            data: app_data,
            checksum,
        })
    }

    fn encode(&self, msg: &Self::Message) -> Result<Bytes> {
        let mut buf = BytesMut::with_capacity(HEADER_SIZE + msg.data.len() + 3);

        // 编码header
        buf.put_u16(msg.header.magic);
        buf.put_u16(msg.header.service_id);
        buf.put_u8(msg.header.version.main_version);
        buf.put_u8(msg.header.version.user_version);
        buf.put_u8(msg.header.timestamp.second);
        buf.put_u8(msg.header.timestamp.minute);
        buf.put_u8(msg.header.timestamp.hour);
        buf.put_u8(msg.header.timestamp.day);
        buf.put_u8(msg.header.timestamp.month);
        buf.put_u8(msg.header.timestamp.year);
        buf.put_slice(&msg.header.src_addr);
        buf.put_slice(&msg.header.dst_addr);
        buf.put_u16(msg.header.data_unit_len);
        buf.put_u8(msg.header.control);

        // 数据
        buf.put_slice(&msg.data);

        // 校验和结束符
        buf.put_u8(msg.checksum);
        buf.put_u16(END_MARKER);

        Ok(buf.freeze())
    }
}

fn parse_header(data: &mut Bytes) -> Result<ProtocolAHeader> {
    let magic = data.get_u16();
    if magic != MAGIC_NUMBER {
        return Err(ProtocolError::InvalidMagic {
            expected: MAGIC_NUMBER,
            got: magic,
        });
    }

    Ok(ProtocolAHeader {
        magic,
        service_id: data.get_u16(),
        version: ProtocolVersion {
            main_version: data.get_u8(),
            user_version: data.get_u8(),
        },
        timestamp: Timestamp {
            second: data.get_u8(),
            minute: data.get_u8(),
            hour: data.get_u8(),
            day: data.get_u8(),
            month: data.get_u8(),
            year: data.get_u8(),
        },
        src_addr: {
            let mut addr = [0u8; 6];
            data.copy_to_slice(&mut addr);
            addr
        },
        dst_addr: {
            let mut addr = [0u8; 6];
            data.copy_to_slice(&mut addr);
            addr
        },
        data_unit_len: data.get_u16(),
        control: data.get_u8(),
    })
}
