use std::sync::Arc;
use async_std::{stream::StreamExt, sync::Mutex, task, net::{TcpStream, TcpListener}};
use web_chat::utils::AppResult;

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
                //log_error p577
            });
        }

        Ok(())
    })
}

async fn serve(stream: TcpStream, groups: Arc<Groups>) -> AppResult<()>
{
    let outbound = Arc::new(Outbound::new(stream.clone()));


    Ok(())
}

fn log_error(result: AppResult<()>) {
    if let Err(error) = result {
        eprintln!("error: {}", error);
    }
}

pub struct Outbound(Mutex<TcpStream>);

impl Outbound
{
    fn new(clone: TcpStream) -> _
    {
        todo!()
    }
}