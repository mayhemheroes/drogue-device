use crate::driver::tls::extensions::server::ServerExtension;

use crate::driver::tls::parse_buffer::ParseBuffer;
use crate::driver::tls::TlsError;
use heapless::{consts::*, Vec};

#[derive(Debug)]
pub struct EncryptedExtensions {
    extensions: Vec<ServerExtension, U16>,
}

impl EncryptedExtensions {
    pub fn parse(buf: &mut ParseBuffer) -> Result<Self, TlsError> {
        //let extensions_len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
        let extensions_len = buf
            .read_u16()
            .map_err(|_| TlsError::InvalidExtensionsLength)?;
        log::info!("extensions length: {}", extensions_len);
        let extensions = ServerExtension::parse_vector(buf)?;
        Ok(Self { extensions })
    }
}