use bytes::BytesMut;
use prost::Message;
use crate::error_handling::Result;

pub fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
    println!("{:#?}", msg);
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
//    msg.encode(&mut buf).map_err(|_err| Error::ProtoError)?;
        msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}
