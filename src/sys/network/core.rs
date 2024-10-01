use reqwest::{
    blocking::{Client, Response},
    header::CONTENT_TYPE,
};

pub fn post(url: String, json: String) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    match client
        .post(url.clone())
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send()
    {
        Ok(response) => Ok(response),
        Err(error) => Err(error),
    }
}
