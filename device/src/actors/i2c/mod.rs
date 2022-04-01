use crate::traits::i2c::I2cAddress;
use crate::{Actor, Address, Inbox};
use core::future::Future;
use embedded_hal_async::i2c::*;

pub struct I2cPeripheral<I: I2c + 'static>
where
    <I as ErrorType>::Error: Send,
{
    i2c: I,
}

pub enum I2cRequest<'m> {
    Read(I2cAddress, &'m mut [u8]),
    Write(I2cAddress, &'m [u8]),
    WriteRead(I2cAddress, &'m [u8], &'m mut [u8]),
    Transaction(I2cAddress, &'m mut [embedded_hal_async::i2c::Operation<'m>]),
}
impl<I: I2c> I2cPeripheral<I>
where
    <I as ErrorType>::Error: Send,
{
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }
}

impl<I: I2c + 'static> Actor for I2cPeripheral<I>
where
    <I as ErrorType>::Error: Send,
{
    type Message<'m> = I2cRequest<'m>;

    type Response = Option<Result<(), <I as ErrorType>::Error>>;

    type OnMountFuture<'m, M> = impl Future<Output = ()> + 'm where Self: 'm, M: 'm + Inbox<Self>;

    fn on_mount<'m, M>(
        &'m mut self,
        _: Address<Self>,
        inbox: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<Self> + 'm,
    {
        async move {
            loop {
                if let Some(mut m) = inbox.next().await {
                    let response = match m.message() {
                        I2cRequest::Read(address, buffer) => {
                            let address: u8 = (*address).into();
                            self.i2c.read(address, buffer).await
                        }
                        I2cRequest::Write(address, bytes) => {
                            let address: u8 = (*address).into();
                            self.i2c.write(address, bytes).await
                        }
                        I2cRequest::WriteRead(address, bytes, buffer) => {
                            let address: u8 = (*address).into();
                            self.i2c.write_read(address, bytes, buffer).await
                        }
                        I2cRequest::Transaction(address, operations) => {
                            let address: u8 = (*address).into();
                            self.i2c.transaction(address, operations).await
                        }
                    };
                    m.set_response(Some(response));
                }
            }
        }
    }
}

pub struct I2cHandle<I: I2c<SevenBitAddress> + 'static>
where
    <I as ErrorType>::Error: Send,
{
    address: Address<I2cPeripheral<I>>,
}

impl<I: I2c<SevenBitAddress> + 'static> embedded_hal_1::i2c::ErrorType for I2cHandle<I>
where
    <I as ErrorType>::Error: Send,
{
    type Error = <I as ErrorType>::Error;
}

impl<I: I2c<SevenBitAddress> + 'static> I2c for I2cHandle<I>
where
    <I as ErrorType>::Error: Send,
{
    type ReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where I: 'a;

    fn read<'a>(&'a mut self, address: u8, buffer: &'a mut [u8]) -> Self::ReadFuture<'a> {
        async move {
            self.address
                .request(I2cRequest::Read(address.into(), buffer))
                .unwrap()
                .await
                .unwrap()
        }
    }

    type WriteFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where I: 'a;
    fn write<'a>(&'a mut self, address: u8, bytes: &'a [u8]) -> Self::WriteFuture<'a> {
        async move {
            self.address
                .request(I2cRequest::Write(address.into(), bytes))
                .unwrap()
                .await
                .unwrap()
        }
    }

    type WriteReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where I: 'a;

    fn write_read<'a>(
        &'a mut self,
        address: u8,
        bytes: &'a [u8],
        buffer: &'a mut [u8],
    ) -> Self::WriteReadFuture<'a> {
        async move {
            self.address
                .request(I2cRequest::WriteRead(address.into(), bytes, buffer))
                .unwrap()
                .await
                .unwrap()
        }
    }

    type TransactionFuture<'a, 'b> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a, 'b: 'a;

    fn transaction<'a, 'b>(
        &'a mut self,
        address: u8,
        operations: &'a mut [embedded_hal_async::i2c::Operation<'b>],
    ) -> Self::TransactionFuture<'a, 'b> {
        let _ = address;
        let _ = operations;
        async move { todo!() }
        /*
        async move {
             * self.request(I2cRequest::Transaction(address.into(), operations))
            .unwrap()
            .await
            .unwrap()
        }
         */
    }
}
