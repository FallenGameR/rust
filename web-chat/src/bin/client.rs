use async_std::prelude::*;
use async_std::{io, net};
use web_chat::ClientPacket;
use web_chat::utils::{AppResult};

fn main(){
}

// was send_commands
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

    }

    Ok(())
}


/*
fn parse_command(line: &str) -> Option<ClientPacket>
{
    let (token, leftover) = get_next_token(line)?;
    if token == "post" {
        let (group, rest) = get_next_token(leftover)?;
        let message = rest.trim_start().to_string();
        return Some(FromClient::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        });
    } else if token == "join" {
        let (group, rest) = get_next_token(leftover)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        return Some(FromClient::Join {
            group_name: Arc::new(group.to_string()),
        });
    } else {
        eprintln!("Unrecognized command: {:?}", line);
        return None;
    }
}
*/

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
fn test_get_next_token_is_correct()
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

    let any_empty_space = "   ";
    let result = get_next_token(any_empty_space);
    assert_eq!(None, result);

    let any_empty_string = "";
    let result = get_next_token(any_empty_string);
    assert_eq!(None, result);
}