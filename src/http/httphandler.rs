use reqwest::{Client, Error, Method};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

pub async fn make_request(url: String, body: String, method: Method) -> Result<String, Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    println!("response: {}", body);

    {
        let this = client.request(method, &url).headers(headers).body(body).send().await;
        match this {
            Ok(response) => {
                let response_body = response.text().await;
                match response_body {
                    Ok(t) => return Ok(t),
                    Err(e) => panic!("{:?}", e),
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}