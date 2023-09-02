use std::io;

use async_trait::async_trait;
use futures::prelude::*;
use futures::{AsyncRead, AsyncWrite};
use libp2p::request_response::{self, Codec, ProtocolSupport};
use libp2p::StreamProtocol;
use prost::Message;

use celestia_proto::p2p::pb::HeaderRequest;
use celestia_proto::p2p::pb::HeaderResponse;

/// Max request size in bytes
const REQUEST_SIZE_MAXIMUM: u64 = 1024;
/// Max response size in bytes
const RESPONSE_SIZE_MAXIMUM: u64 = 10 * 1024 * 1024;

pub type ExchangeBehaviour = request_response::Behaviour<HeaderCodec>;

/// Create a new [`ExchangeBehaviour`]
pub fn exchange_behaviour(network: &str) -> ExchangeBehaviour {
    let protocol = format!("/{network}/header-ex/v0.0.3");
    request_response::Behaviour::new(
        [(
            StreamProtocol::try_from_owned(protocol).expect("starts from '/'"),
            ProtocolSupport::Full,
        )],
        request_response::Config::default(),
    )
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeaderCodec;

#[async_trait]
impl Codec for HeaderCodec {
    type Protocol = StreamProtocol;
    type Request = HeaderRequest;
    type Response = HeaderResponse;

    async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut vec = Vec::new();

        io.take(REQUEST_SIZE_MAXIMUM).read_to_end(&mut vec).await?;

        Ok(HeaderRequest::decode_length_delimited(&vec[..])?)
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut vec = Vec::new();

        io.take(RESPONSE_SIZE_MAXIMUM).read_to_end(&mut vec).await?;

        Ok(HeaderResponse::decode_length_delimited(&vec[..])?)
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = req.encode_length_delimited_to_vec();

        io.write_all(data.as_ref()).await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        resp: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = resp.encode_length_delimited_to_vec();

        io.write_all(data.as_ref()).await?;

        Ok(())
    }
}
