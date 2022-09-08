use std::sync::Arc;
use async_std::{stream::StreamExt, task};
use web_chat::utils::AppResult;

mod connection;
mod groups;
mod group;

use crate::groups::Groups;

fn main() -> AppResult<()>
{
    let server_address  = std::env::args().nth(1).expect("Usage: server <SERVER ADDRESS>:<PORT>");
    let groups = Arc::new(Groups::new());

    async_std::task::block_on(async {
        let listner = async_std::net::TcpListener::bind(server_address).await?;
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

fn log_error(result: AppResult<()>) {
    if let Err(error) = result {
        eprintln!("error: {}", error);
    }
}