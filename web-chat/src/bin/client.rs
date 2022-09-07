use std::sync::Arc;

use async_std::prelude::*;
use async_std::{io, net};
use web_chat::{ClientPacket, ServerPacket, utils};
use web_chat::utils::{AppResult};

fn main() -> AppResult<()>
{
    let address = std::env::args().nth(1).expect("Usage: client.exe <ADDRESS>:<PORT>");

    async_std::task::block_on(async {
        let server_stream = net::TcpStream::connect(address).await?;
        server_stream.set_nodelay(true)?;

        let sent_to_server = send_packet(server_stream.clone());
        let replied_from_server = receive_packet(server_stream);

        replied_from_server.race(sent_to_server).await?;
        Ok(())
    })
}

// was send_commands
// to test this we'll need to depend on trait instead
// and test would pass in mock struct that implements the same trait
async fn send_packet(mut server: net::TcpStream) -> AppResult<()>
{
    println!(
        "# Awailable commands\n\
        - J group_name - join chat group with that name\n\
        - S group_name message_text - send chat group with that name the message\n\
        - Ctrl+Z - close connection and exit the client app");

    let mut input = io::BufReader::new(io::stdin()).lines();

    while let Some(line_read) = input.next().await {
        let line = line_read?;

        let packet = match command_to_packet(&line) {
            Some(packet) => packet,
            None => continue,
        };

        utils::send_packet(&mut server, &packet).await?;
        server.flush().await?;
    }

    Ok(())
}

// was: handle_replies
async fn receive_packet(server: net::TcpStream) -> AppResult<()>
{
    let reader = io::BufReader::new(server);
    let mut stream = utils::receive_packet(reader);

    while let Some(packet) = stream.next().await {
        match packet? {
            ServerPacket::Message{ group, message } => {
                println!("{}: {}", group, message);
            }
            ServerPacket::Error(message) => {
                eprintln!("error: server replied with error message: {}", message)
            }
        }
    }

    Ok(())
}

// was: parse_command
fn command_to_packet(line: &str) -> Option<ClientPacket>
{
    let (token, leftover) = get_next_token(line)?;

    match token {
        "J" => {
            // Join group
            let (group, leftover) = get_next_token(leftover)?;
            if !leftover.trim_start().is_empty() {
                eprintln!("Error: Incorrect join command arguments. Should be 'J group_name'.");
                return None;
            }
            return Some(ClientPacket::Join {
                group: Arc::new(group.to_string()),
            });
        },
        "S" => {
            // Send message to group
            let (group, message) = get_next_token(leftover)?;
            return Some(ClientPacket::Send {
                group: Arc::new(group.to_string()),
                message: Arc::new(message.trim_start().to_string()),
            });
        },
        _ => {
            eprintln!("Error: Unrecognized command: {:?}", line);
            return None;
        }
    }
}

#[test]
fn test_command_to_packet()
{
    // Joins
    let any_valid_join = command_to_packet("  J cats").unwrap();
    assert_eq!(ClientPacket::Join { group: Arc::new("cats".to_string()) }, any_valid_join);

    let any_no_group_join = command_to_packet("J ");
    assert_eq!(None, any_no_group_join);

    // Sends
    let any_valid_send = command_to_packet("S cats hello and myau!").unwrap();
    let any_matching_send_packet = ClientPacket::Send {
        group: Arc::new("cats".to_string()),
        message: Arc::new("hello and myau!".to_string()),
    };
    assert_eq!(any_matching_send_packet, any_valid_send);

    let any_no_message_send = command_to_packet("S cats ").unwrap();
    let any_matching_send_packet = ClientPacket::Send {
        group: Arc::new("cats".to_string()),
        message: Arc::new("".to_string()),
    };
    assert_eq!(any_matching_send_packet, any_no_message_send);

    let any_no_group_send = command_to_packet("S ");
    assert_eq!(None, any_no_group_send);

    // Unknown commands
    let any_unknown_command = command_to_packet("List database");
    assert_eq!(None, any_unknown_command);
}

/// Return 'Some((first_word, leftover_text))' if there is
/// at least one word in the 'text', otherwise return 'None'
fn get_next_token(mut text: &str) -> Option<(&str, &str)>
{
    text = text.trim_start();

    if text.is_empty() {
        return None;                                // None enum option
    }

    match text.find(char::is_whitespace) {
        Some(i) => Some((&text[0..i], &text[i..])), // tuple with two string slices
        None => Some((text, "")),                   // tuple with one string slice from text
                                                    //   and a fat pointer to an empty string slice
    }
}

#[test]
fn test_get_next_token()
{
    let (any_token, any_text_sans_token) = get_next_token("someToken next text here").unwrap();
    assert_eq!("someToken", any_token);
    assert_eq!(" next text here", any_text_sans_token);

    let (any_token, any_text_sans_token) = get_next_token(any_text_sans_token).unwrap();
    assert_eq!("next", any_token);
    assert_eq!(" text here", any_text_sans_token);

    let (any_last_token, any_text_sans_token) = get_next_token("lastToken ").unwrap();
    assert_eq!("lastToken", any_last_token);
    assert_eq!(" ", any_text_sans_token);

    let (any_last_token, empty_text) = get_next_token("justLastToken").unwrap();
    assert_eq!("justLastToken", any_last_token);
    assert_eq!("", empty_text);

    let any_empty_space = "  ";
    let result = get_next_token(any_empty_space);
    assert_eq!(None, result);

    let any_empty_string = "";
    let result = get_next_token(any_empty_string);
    assert_eq!(None, result);
}
