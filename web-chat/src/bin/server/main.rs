use std::sync::Arc;
use async_std::{
    stream::StreamExt,
    sync::Mutex,
    task,
    net::{
        TcpStream,
        TcpListener
    },
    io::{
        BufReader,
        WriteExt
    }
};
use web_chat::{utils::{AppResult, self}, ClientPacket, ServerPacket};

mod groups;

use crate::groups::Groups;


fn main() -> AppResult<()>
{
    let server_address  = std::env::args().nth(1).expect("Usage: server <SERVER ADDRESS>:<PORT>");
    let groups = Arc::new(Groups::new());

    async_std::task::block_on(async {
        let listner = TcpListener::bind(server_address).await?;
        let mut connections = listner.incoming();

        while let Some(tcp_stream_result) = connections.next().await {
            let tcp_stream = tcp_stream_result?;
            let groups_for_task = groups.clone();
            task::spawn(async {
                let server_termination_reason = process_packets(tcp_stream, groups_for_task).await;
                if let Err(message) = server_termination_reason {
                    eprintln!("error: {}", message);
                }
                else {
                    eprintln!("closed: connection was closed");
                }
            });
        }

        Ok(())
    })
}

async fn process_packets(stream: TcpStream, groups: Arc<Groups>) -> AppResult<()>
{
    let server_reply_stream = Arc::new(Outbound::new(stream.clone()));
    let client_read_stream = BufReader::new(stream);
    let mut client_read_packets_stream = utils::receive_packet(client_read_stream);

    while let Some(client_read_packet_result) = client_read_packets_stream.next().await  {
        let client_packet_processing_result = match client_read_packet_result? {
            ClientPacket::Join { group } => {
                let used_group = groups.get_or_create(group);
                used_group.join(server_reply_stream.clone());   // reply stream is needed in post
                Ok(())
            }
            ClientPacket::Send { group, message } => {
                match groups.get(&group) {
                    Some(used_group) => {
                        used_group.post(message);               // would use preserved stream
                        Ok(())
                    }
                    None => {
                        Err(format!(
                            "Can't send message '{}' to the group '{}' \
                            because the group does not exist",
                            message, group))
                    }

                }
            }
        };

        if let Err(message) = client_packet_processing_result {
            let error_reply = ServerPacket::Error(message);
            server_reply_stream.send(error_reply).await?;
        }
    }

    Ok(())
}

// Same TcpStream can be used by the server
// to reply simualtaneously to multiple clients.
// Thus a mutex guard is needed to prevent races.
pub struct Outbound(Mutex<TcpStream>);

impl Outbound
{
    fn new(stream: TcpStream) -> Outbound
    {
        Outbound(Mutex::new(stream))
    }

    async fn send(&self, packet: ServerPacket) -> AppResult<()>
    {
        let mut guarded_stream = self.0.lock().await;
        utils::send_packet(&mut *guarded_stream, &packet).await?;
        guarded_stream.flush().await?;
        Ok(())
    }
}