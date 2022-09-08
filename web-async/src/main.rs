use async_std::io::prelude::*;
use async_std::net;
use std::io::Result;

/* Example of execution:

> web_request(example.com): enter
> web_request(www.red-bean.com): enter
> web_request(en.wikipedia.org): enter
> web_request(example.com): connect
> web_request(example.com): send
> web_request(en.wikipedia.org): connect
> web_request(en.wikipedia.org): send
> web_request(example.com): receive
> web_request_owning(example.com): web_request.await
> web_requests: handle.await
> web_request(en.wikipedia.org): receive
> web_request_owning(en.wikipedia.org): web_request.await
> web_request(www.red-bean.com): connect
> web_request(www.red-bean.com): send
> web_request(www.red-bean.com): receive
> web_request_owning(www.red-bean.com): web_request.await
> web_requests: handle.await
> web_requests: handle.await

*/
fn main() {
    let requests = vec![
        ("example.com".to_string(),         80,     "/".to_string()),
        ("www.red-bean.com".to_string(),    80,     "/".to_string()),
        ("en.wikipedia.org".to_string(),    80,     "/".to_string()),
    ];

    let results = async_std::task::block_on(web_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(error) => eprintln!("error: {}", error),
        }
    }
}

/*
fn main() -> Result<()> {
    let response = async_std::task::block_on(web_request("example.com", 80, "/"))?;
    println!("{}", response);
    Ok(())
}
*/

async fn web_requests(requests: Vec<(String, u16, String)>) -> Vec<Result<String>> {
    let mut handles = vec![];
    for (host, port, path) in requests {
        // handles.push(async_std::task::spawn_local(web_request_owning(host, port, path)));

        handles.push(async_std::task::spawn_local(async move {  // this block returns future
            web_request(&host, port, &path).await               // all the used variable moved
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
        eprintln!("> web_requests: handle.await finished");
    }

    results
}

/*
async fn web_request_owning(host: String, port: u16, path: String) -> Result<String> {
    let result = web_request(&host, port, &path).await;
    eprintln!("> web_request_owning({}): web_request.await", host);
    result
}
*/

async fn web_request(host: &str, port: u16, path: &str) -> Result<String> {
    eprintln!("> web_request({}): just entered", host);

    let mut socket = net::TcpStream::connect((host, port)).await?;
    eprintln!("> web_request({}): connected", host);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    eprintln!("> web_request({}): sended", host);
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(& mut response).await?;
    eprintln!("> web_request({}): received", host);

    Ok(response)
}