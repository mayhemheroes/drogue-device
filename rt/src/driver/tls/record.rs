use crate::api::ip::tcp::{TcpSocket, TcpStack};
use crate::driver::tls::application_data::ApplicationData;
use crate::driver::tls::change_cipher_spec::ChangeCipherSpec;
use crate::driver::tls::config::Config;
use crate::driver::tls::content_types;
use crate::driver::tls::content_types::ContentType;
use crate::driver::tls::handshake::client_hello::ClientHello;
use crate::driver::tls::handshake::{ClientHandshake, HandshakeType, ServerHandshake};
use crate::driver::tls::TlsError;
use heapless::{consts::*, ArrayLength, Vec};
use rand_core::{CryptoRng, RngCore};
use sha2::Digest;

pub enum ClientRecord<'config, R, D>
where
    R: CryptoRng + RngCore + Copy,
    D: Digest,
{
    Handshake(ClientHandshake<'config, R, D>),
    ApplicationData(ApplicationData),
}

impl<'config, R, D> ClientRecord<'config, R, D>
where
    R: CryptoRng + RngCore + Copy,
    D: Digest,
{
    pub fn content_type(&self) -> ContentType {
        match self {
            ClientRecord::Handshake(_) => ContentType::Handshake,
            ClientRecord::ApplicationData(_) => ContentType::ApplicationData,
        }
    }

    pub fn client_hello(config: &'config Config<R>) -> Self {
        ClientRecord::Handshake(ClientHandshake::ClientHello(ClientHello::new(config)))
    }

    pub fn encode<N: ArrayLength<u8>>(
        &self,
        buf: &mut Vec<u8, N>,
        digest: &mut D,
    ) -> Result<(), TlsError> {
        match self {
            ClientRecord::Handshake(_) => {
                buf.push(ContentType::Handshake as u8);
                buf.extend_from_slice(&[0x03, 0x01]);
            }
            ClientRecord::ApplicationData(_) => {
                buf.push(ContentType::ApplicationData as u8);
                buf.extend_from_slice(&[0x03, 0x03]);
            }
        }

        let record_length_marker = buf.len();
        buf.push(0);
        buf.push(0);

        match self {
            ClientRecord::Handshake(handshake) => {
                /*
                match handshake {
                    ClientHandshake::ClientHello(_) => {
                        buf.push(HandshakeType::ClientHello as u8);
                    }
                    ClientHandshake::Finished(_) => {
                        buf.push(HandshakeType::Finished as u8);
                    }
                }
                 */
                handshake.encode(buf, digest)?;
            }
            ClientRecord::ApplicationData(application_data) => {
                buf.extend(application_data.data.iter());
            }
        }

        let record_length = (buf.len() as u16 - record_length_marker as u16) - 2;

        log::info!("record len {}", record_length);

        buf[record_length_marker] = record_length.to_be_bytes()[0];
        buf[record_length_marker + 1] = record_length.to_be_bytes()[1];

        Ok(())
    }
}

#[derive(Debug)]
pub enum ServerRecord<D: Digest> {
    Handshake(ServerHandshake<D>),
    ChangeCipherSpec(ChangeCipherSpec),
    Alert,
    ApplicationData(ApplicationData),
}

impl<D: Digest> ServerRecord<D> {
    pub async fn read<T: TcpStack>(
        socket: &mut TcpSocket<T>,
        digest: &mut D,
    ) -> Result<Self, TlsError> {
        let mut header = [0; 5];
        let mut pos = 0;
        loop {
            pos += socket.read(&mut header[pos..]).await?;
            if pos == header.len() {
                break;
            }
        }

        log::info!("receive header {:x?}", &header);

        match ContentType::of(header[0]) {
            None => Err(TlsError::InvalidRecord),
            Some(content_type) => {
                let content_length = u16::from_be_bytes([header[3], header[4]]);
                match content_type {
                    ContentType::Invalid => Err(TlsError::Unimplemented),
                    ContentType::ChangeCipherSpec => Ok(ServerRecord::ChangeCipherSpec(
                        ChangeCipherSpec::read(socket, content_length).await?,
                    )),
                    ContentType::Alert => Err(TlsError::Unimplemented),
                    ContentType::Handshake => Ok(ServerRecord::Handshake(
                        ServerHandshake::read(socket, content_length, digest).await?,
                    )),
                    ContentType::ApplicationData => Ok(ServerRecord::ApplicationData(
                        ApplicationData::read(socket, content_length, &header).await?,
                    )),
                }
            }
        }
    }

    //pub fn parse<D: Digest>(buf: &[u8]) -> Result<Self, TlsError> {}
}
