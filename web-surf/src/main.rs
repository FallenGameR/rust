//use surf::http::response;

fn main() {
    let urls = &[
        "http://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org/wiki/Main_Page".to_string(),
    ];

    let results = async_std::task::block_on(many_requests(urls));
    for result in results {
        match result {
            Ok(response) => println!("*** {}\n", response),
            Err(error) => eprintln!("error: {}\n", error),
        }
    }
}

async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}