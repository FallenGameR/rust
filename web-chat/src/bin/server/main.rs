use std::sync::Arc;
use async_std::{
    stream::StreamExt,
    sync::Mutex,
    task,
    net::{
        TcpStream,
        TcpListener,
    },
    io::{
        BufReader,
        WriteExt,
    }
};
use web_chat::{
    utils::{
        self,
        AppResult,
    },
    ClientPacket,
    ServerPacket
};

// this is not web_chat crate but rather bin/server crate inside web_chat
use crate::groups::Groups;

mod groups;

fn main() -> AppResult<()>
{
    // Shared across the server app
    let groups = Arc::new(Groups::new());

    async_std::task::block_on(async {
        // was: places outside of async block
        let server_address = std::env::args().nth(1).expect("Usage: server <SERVER ADDRESS>:<PORT>");

        // this is really a tcp socket server and original code calls it socket
        let listner = TcpListener::bind(server_address).await?;

        while let Some(tcp_stream_result) = listner.incoming().next().await {
            let tcp_stream = tcp_stream_result?;
            let groups_copy = groups.clone();

            // async task that is spawn for each connection
            // the tcp_streams would be shared via the groups that would remember
            // what connection to use for replies
            task::spawn(async {
                let server_termination_reason = process_packets(tcp_stream, groups_copy).await;
                if let Err(message) = server_termination_reason {
                    eprintln!("error: {}", message);
                }
                else {
                    println!("client connection was closed");
                }
            });
        }

        Ok(())
    })
}

// was: serve
async fn process_packets(stream: TcpStream, groups: Arc<Groups>) -> AppResult<()>
{
    // All replies to that connected to the servier client
    // go through that guarded reply stream
    let server_reply_stream = Arc::new(Outbound::new(stream.clone()));

    // reads from the client stream are all handled within this function
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

    // NOTE: server_reply_stream here is useless - the connection is closed
    // so we actually need to remove this stream from all the groups that use it
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
        // async_std's Mutex (it is not from std) is used since we are working with async functions:
        // 1) it would work if the same task tries to re-lock it again
        // 2) it uses future to yield the thread if mutex was alteady taken by
        // somebody else if nobody took the mutex there is no thread yield
        // 3) async mutex can be released by a different thread, not the one
        // that locked it, that is common in async functions
        Outbound(Mutex::new(stream))
    }

    async fn send(&self, packet: ServerPacket) -> AppResult<()>
    {
        let mut guarded_stream = self.0.lock().await;

        // This &mut * syntax is mitigation to the fact that Rust
        // doesn't do deref coercions to satisfy trait bounds.
        //
        // So we explicitly dereferencing the mutex quard and then
        // borrow a mutable reference to the protected TCP stream.
        //
        // Dereference has the highest precedence
        utils::send_packet(&mut *guarded_stream, &packet).await?;
        guarded_stream.flush().await?;
        Ok(())
    }
}