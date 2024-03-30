use reqwest::{Client, Error};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

pub async fn make_request(url: String, body: String) -> Result<String, Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    {
        let this = client.post(&url).headers(headers).body(body).send().await;
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