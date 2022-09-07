use async_std::{io::{WriteExt, prelude::BufReadExt}, stream::StreamExt};


// In real apps use anyhow crate for generic thread-safe errors
// p568
pub type AppError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type AppResult<T> = Result<T, AppError>;

pub async fn send_packet<Stream, Packet>(outbound: &mut Stream, packet: &Packet) -> AppResult<()>
where
    Stream: async_std::io::Write + Unpin,
    Packet: serde::Serialize
{
    let mut json = serde_json::to_string(&packet)?;

    // New line is used to separate commands for processing
    json.push('\n');

    // We could have called flush here as well
    // Right now this util method assumes that buffer is flushed upstream
    outbound.write_all(json.as_bytes()).await?;

    Ok(())
}

pub fn receive_packet<Stream, Packet>(inbound: Stream) -> impl async_std::prelude::Stream<Item = AppResult<Packet>>
where
    Stream: async_std::io::BufRead + Unpin,
    Packet: serde::de::DeserializeOwned
{
    inbound.lines().map(|line_read| -> AppResult<Packet> {
        let line = line_read?;
        let packet = serde_json::from_str::<Packet>(&line)?;
        Ok(packet)
    })
}