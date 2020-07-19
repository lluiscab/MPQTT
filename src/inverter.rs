use crate::crc::digest_crc;
use std::mem::size_of;
use tokio::io::{Result, BufStream};
use tokio::prelude::*;
use std::io::BufRead;

pub struct Inverter<T> {
    inner: BufStream<T>,
}

impl<T> Inverter<T>
    where
        T: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(inner: T) -> Self {
        Self { inner: BufStream::new(inner) }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    // TODO: Change asserts to custom Result (més endavant...).
    pub async fn query_serial_number(&mut self) -> Result<()> {
        // Send the command.
        self.write_cmd(b"QID").await?;
        self.inner.flush().await?;

        let mut buf = Vec::new();
        self.inner.read_until('\r' as u8, &mut buf).await?;

        unimplemented!()
    }

    async fn write_cmd(&mut self, cmd: &[u8]) -> Result<()> {
        let hash = digest_crc(cmd);
        let mut buf = Vec::with_capacity(cmd.len() + size_of::<u16>() + size_of::<u8>());

        // Write the command.
        buf.extend_from_slice(cmd);

        // Write the CRC sum.
        buf.extend_from_slice(hash.to_be_bytes().as_ref());

        // Write the carriage return.
        buf.extend_from_slice(&['\r' as u8]);

        // Send the command.
        self.inner.write(buf.as_slice()).await?;
        println!("Data sent: {:?}", buf);

        Ok(())
    }
}