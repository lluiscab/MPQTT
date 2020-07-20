pub mod error;
pub mod util;

use self::error::{Error, Result};
use crate::crc::digest_crc;
use std::mem::size_of;
use std::str::FromStr;
use tokio::io::BufStream;
use tokio::prelude::*;

/// An inverter interface.
#[derive(Debug)]
pub struct Inverter<T>(BufStream<T>);

impl<T> Inverter<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    /// Create a new `Inverter` interface from an io stream.
    pub fn from_stream(inner: T) -> Self {
        Self(BufStream::new(inner))
    }

    /// Return the io stream used to create the `Inverter` interface.
    #[allow(unused)]
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    /// Query and return the device protocol id.
    #[allow(unused)]
    pub async fn query_protocol_id(&mut self) -> Result<usize> {
        // Send the command.
        self.write_cmd(b"QPI").await?;
        self.0.flush().await?;

        // Read the response.
        let res = self.read_response_line().await?;

        // Parse the response.
        let (prefix, payload) = res.split_at(2);
        if prefix != b"PI" {
            return Err(Error::InvalidResponsePayload);
        }

        let payload = std::str::from_utf8(payload)?;
        Ok(usize::from_str(payload)?)
    }

    /// Query and return the serial number of the inverter.
    #[allow(unused)]
    pub async fn query_serial_number(&mut self) -> Result<usize> {
        // Send the command.
        self.write_cmd(b"QID").await?;

        // Read the response.
        let res = self.read_response_line().await?;

        // Parse the response.
        unimplemented!()
        // match usize::from_str(String::from_utf8(res)?.as_str()) {
        //     Ok(x) => Ok(x),
        //     Err(e) => Err(Error::from(e)),
        // }
    }

    /// Read a single response line.
    ///
    /// Bytes are considered on the same line until a carriage return is read.
    async fn read_response_line(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.0.read_until('\r' as u8, &mut buf).await?;

        // Common response line checks.
        if buf.len() < (size_of::<u8>() + size_of::<u16>() + size_of::<u8>()) {
            return Err(Error::InvalidResponseFormat);
        }

        // Check the checksum.
        let payload = buf[1..buf.len() - 3].to_vec();
        let payload_hash = digest_crc(&payload);

        let hash = &buf[buf.len() - 3..buf.len() - 1];
        if payload_hash.to_be_bytes().as_ref() != hash {
            return Err(Error::InvalidResponseCheckSum);
        }

        Ok(payload)
    }

    /// Send a command.
    async fn write_cmd(&mut self, cmd: &[u8]) -> Result<()> {
        let hash = digest_crc(cmd);
        let mut buf = Vec::with_capacity(cmd.len() + size_of::<u16>() + size_of::<u8>());

        // Write the command.
        // Write the CRC sum.
        // Write the carriage return.
        buf.extend_from_slice(cmd);
        buf.extend_from_slice(hash.to_be_bytes().as_ref());
        buf.extend_from_slice(&['\r' as u8]);

        // Send the command.
        self.0.write_all(buf.as_slice()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::crc::digest_crc;
    use crate::inverter::Inverter;
    use futures::future::{select, Either};
    use futures_ringbuf::TokioEndpoint;
    use std::mem::size_of;
    use tokio::prelude::*;
    use tokio::sync::oneshot::{channel, Receiver};

    const MOCK_BUFFER_LEN: usize = 1024;

    // Mock inverter return parameters.
    const MOCK_PROTOCOL_ID: usize = 1234;
    const MOCK_SERIAL_NUMBER: usize = 0xdeadbeef;

    async fn run_server<S>(mut stream: S, mut stopper: Receiver<()>)
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        fn mk_res<T: AsRef<[u8]>>(payload: T) -> Vec<u8> {
            let payload = payload.as_ref();
            let hash = digest_crc(&payload);

            let mut data = Vec::with_capacity(
                size_of::<u8>() + payload.len() + size_of::<u16>() + size_of::<u8>(),
            );

            data.push('(' as u8);
            data.extend_from_slice(payload);
            data.extend_from_slice(&hash.to_be_bytes());
            data.push('\r' as u8);

            data
        }

        let mock_res_qpi = mk_res(format!("PI{}", MOCK_PROTOCOL_ID));
        let mock_res_qid = mk_res(format!("{}", MOCK_SERIAL_NUMBER));

        let mut buf = Vec::new();

        loop {
            match select(stream.read_u8(), &mut stopper).await {
                Either::Left((x, _)) => {
                    let x = x.unwrap();

                    buf.push(x);

                    if x == '\r' as u8 {
                        match buf.as_slice() {
                            b"QPI\xbe\xac\r" => {
                                stream.write_all(mock_res_qpi.as_slice()).await.unwrap()
                            }
                            b"QID\xd6\xea\r" => {
                                stream.write_all(mock_res_qid.as_slice()).await.unwrap()
                            }
                            b"QVFW\x62\x99\r" => unimplemented!(),
                            b"QVFW2\xc3\xf5\r" => unimplemented!(),
                            b"QPIRI\xf8\x54\r" => unimplemented!(),
                            b"QFLAG\x98\x74\r" => unimplemented!(),
                            b"QPIGS\xb7\xa9\r" => unimplemented!(),
                            b"QMOD\xb7\xa9\r" => unimplemented!(),
                            b"QPIWS\xb4\xda\r" => unimplemented!(),
                            b"QDI\x71\x1b\r" => unimplemented!(),
                            b"QMCHGCR\xd8\x55\r" => unimplemented!(),
                            b"QMUCHGCR\x26\x34\r" => unimplemented!(),
                            b"QBOOT\x0a\x88\r" => unimplemented!(),
                            b"QOPM\xa5\xc5\r" => unimplemented!(),
                            // TODO: Add parallel commands.
                            _ => unimplemented!(),
                        }
                    }
                }
                Either::Right(_) => {
                    break;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_query_protocol_id() -> Result<(), Box<dyn std::error::Error>> {
        let (stream_a, stream_b) = TokioEndpoint::pair(MOCK_BUFFER_LEN, MOCK_BUFFER_LEN);

        // Start the server task asynchronously.
        let (stopper_tx, stopper_rx) = channel();
        let server_task = tokio::spawn(run_server(stream_a, stopper_rx));

        // Run the test.
        let mut inverter = Inverter::from_stream(stream_b);
        assert_eq!(inverter.query_protocol_id().await?, MOCK_PROTOCOL_ID);

        // Stop and await the server task.
        stopper_tx.send(()).unwrap();
        server_task.await?;

        Ok(())
    }

    // #[tokio::test]
    async fn test_query_serial_number() -> Result<(), Box<dyn std::error::Error>> {
        let (stream_a, stream_b) = TokioEndpoint::pair(MOCK_BUFFER_LEN, MOCK_BUFFER_LEN);

        // Start the server task asynchronously.
        let (stopper_tx, stopper_rx) = channel();
        let server_task = tokio::spawn(run_server(stream_a, stopper_rx));

        // Run the test.
        let mut inverter = Inverter::from_stream(stream_b);
        assert_eq!(inverter.query_serial_number().await?, MOCK_PROTOCOL_ID);

        // Stop and await the server task.
        stopper_tx.send(()).unwrap();
        server_task.await?;

        Ok(())
    }
}
