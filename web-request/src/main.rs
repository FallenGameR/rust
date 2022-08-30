use async_std::io::prelude::*;
use async_std::net;

fn main() -> std::io::Result<()> {
    let response = async_std::task::block_on(web_request("example.com", 80, "/"))?;
    println!("{}", response);
    Ok(())
}

async fn web_request(host: &str, port: u16, path: & str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(& mut response).await?;

    Ok(response)
}