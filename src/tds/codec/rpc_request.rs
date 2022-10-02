use super::{AllHeaderTy, Encode, ALL_HEADERS_LEN_TX};
use crate::{tds::codec::ColumnData, BytesMutWithTypeInfo, Result};
use bytes::{BufMut, BytesMut};
use enumflags2::{bitflags, BitFlags};
use std::borrow::BorrowMut;
use std::borrow::Cow;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RpcStatus {
    ByRefValue = 1 << 0,
    DefaultValue = 1 << 1,
    // reserved
    Encrypted = 1 << 3,
}

#[bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RpcOption {
    WithRecomp = 1 << 0,
    NoMeta = 1 << 1,
    ReuseMeta = 1 << 2,
}

#[derive(Debug)]
pub struct TokenRpcRequest<'a> {
    proc_id: RpcProcIdValue<'a>,
    flags: BitFlags<RpcOption>,
    params: Vec<RpcParam<'a>>,
    transaction_desc: [u8; 8],
}

impl<'a> TokenRpcRequest<'a> {
    pub fn new<I>(proc_id: I, params: Vec<RpcParam<'a>>, transaction_desc: [u8; 8]) -> Self
    where
        I: Into<RpcProcIdValue<'a>>,
    {
        Self {
            proc_id: proc_id.into(),
            flags: BitFlags::empty(),
            params,
            transaction_desc,
        }
    }
}

#[derive(Debug)]
pub struct RpcParam<'a> {
    pub name: Cow<'a, str>,
    pub flags: BitFlags<RpcStatus>,
    pub value: ColumnData<'a>,
}

/// 2.2.6.6 RPC Request
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum RpcProcId {
    CursorOpen = 2,
    CursorFetch = 7,
    CursorClose = 9,
    ExecuteSQL = 10,
    Prepare = 11,
    Execute = 12,
    PrepExec = 13,
    Unprepare = 15,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RpcProcIdValue<'a> {
    Name(Cow<'a, str>),
    Id(RpcProcId),
}

impl<'a, S> From<S> for RpcProcIdValue<'a>
where
    S: Into<Cow<'a, str>>,
{
    fn from(s: S) -> Self {
        Self::Name(s.into())
    }
}

impl<'a> From<RpcProcId> for RpcProcIdValue<'a> {
    fn from(id: RpcProcId) -> Self {
        Self::Id(id)
    }
}

impl<'a> Encode<BytesMut> for TokenRpcRequest<'a> {
    fn encode(self, dst: &mut BytesMut) -> Result<()> {
        log::debug!("encode(RpcRequest) - {:?} bytes in dst", dst.len());
        log::debug!(" - encoding header");
        dst.put_u32_le(ALL_HEADERS_LEN_TX as u32);
        dst.put_u32_le(ALL_HEADERS_LEN_TX as u32 - 4);
        log::debug!(" - ALL_HEADERS_LEN_TX = {:?}", ALL_HEADERS_LEN_TX);

        log::debug!(" - {:?} bytes in dest", dst.len());
        for chunk in dst.chunks(16) {
            log::warn!("{:?}", hex::encode(chunk))
        }

        dst.put_u16_le(AllHeaderTy::TransactionDescriptor as u16);
        log::debug!(" - transaction descriptor header = {:?}", AllHeaderTy::TransactionDescriptor as u16);

        log::debug!(" - {:?} bytes in dest", dst.len());
        for chunk in dst.chunks(16) {
            log::warn!("{:?}", hex::encode(chunk))
        }

        dst.put_slice(&self.transaction_desc);
        log::debug!(" - transaction descriptor = {:?}", &self.transaction_desc);

        log::debug!(" - {:?} bytes in dest", dst.len());
        for chunk in dst.chunks(16) {
            log::warn!("{:?}", hex::encode(chunk))
        }

        log::debug!(" - writing 1");
        dst.put_u32_le(1);

        log::debug!(" - {:?} bytes in dest", dst.len());
        for chunk in dst.chunks(16) {
            log::warn!("{:?}", hex::encode(chunk))
        }
        match self.proc_id {
            RpcProcIdValue::Id(ref id) => {
                let val = (0xffff_u32) | ((*id as u16) as u32) << 16;
                dst.put_u32_le(val);
            }
            RpcProcIdValue::Name(ref _name) => {
                //let (left_bytes, _) = try!(write_varchar::<u16>(&mut cursor, name, 0));
                //assert_eq!(left_bytes, 0);
                todo!()
            }
        }

        dst.put_u16_le(self.flags.bits() as u16);
        log::debug!(" - encoding params");
        for param in self.params.into_iter() {
            param.encode(dst)?;
        }
        log::debug!(" - done encode");

        Ok(())
    }
}

impl<'a> Encode<BytesMut> for RpcParam<'a> {
    fn encode(self, dst: &mut BytesMut) -> Result<()> {
        log::debug!("encode (RpcParam) - {:?} bytes in dst", dst.len());
        let len_pos = dst.len();
        log::debug!(" - saving length position as {:?}", len_pos);
        let mut length = 0u8;

        dst.put_u8(length);
        log::debug!(" - writing name - {:?} bytes in dst", dst.len());
        for codepoint in self.name.encode_utf16() {
            length += 1;
            dst.put_u16_le(codepoint);
        }
        log::debug!(" - wrote name - {:?} bytes in dst", dst.len());

        dst.put_u8(self.flags.bits());
        log::debug!(" - wrote flags - {:?} bytes in dst", dst.len());

        let mut dst_fi = BytesMutWithTypeInfo::new(dst);
        self.value.encode(&mut dst_fi)?;
        log::debug!(" - wrote value - {:?} bytes in dst", dst.len());

        let dst: &mut [u8] = dst.borrow_mut();
        dst[len_pos] = length;
        log::debug!(" - updated length at {:?} to {:?}", len_pos, length);
        Ok(())
    }
}
