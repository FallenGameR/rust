use async_std::io::WriteExt;


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
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}