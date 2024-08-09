use crate::base;
use crate::base::config;
use crate::modules::json;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

fn post(url: String, json: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = Client::new();
    match client
        .post(url.clone())
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send()
    {
        Ok(response) => Ok(response),
        Err(error) => {
            if error.is_timeout() {
                base::log(
                    &format!("Timeout while sending POST request to {}.", url),
                    1,
                );
            } else if error.is_connect() {
                base::log(
                    &format!("Connection error while sending POST request to {}.", url),
                    1,
                );
            } else {
                base::log(&format!("Error while sending POST request to {}.", url), 1);
            }
            Err(error)
        }
    }
}

pub fn register(uuid: String, publickey: String) -> u16 {
    let data = json::PostRegister { uuid, publickey };
    let json = json::to_string(&data);
    match post(config::url("register"), json) {
        Ok(response) => response.status().as_u16(),
        Err(_) => 0,
    }
}

pub fn get(uuid: String, gettime: String, gettimesignature: String) -> (u16, json::GetResponse) {
    let data = json::PostGet {
        uuid,
        gettime,
        gettimesignature,
    };
    let json = json::to_string(&data);
    match post(config::url("get"), json) {
        Ok(response) => (
            response.status().as_u16(),
            match response.json::<json::GetResponse>() {
                Ok(data) => data,
                Err(err) => {
                    base::log(&format!("Error while parsing JSON response: {}", err), 1);
                    Vec::from([])
                }
            },
        ),
        Err(_) => (0, Vec::from([])),
    }
}

pub fn send(
    receiver: String,
    message: json::Message,
    sendtime: String,
    sendtimesignature: String,
) -> u16 {
    let data = json::PostSend {
        receiver,
        message,
        sendtime,
        sendtimesignature,
    };
    let json = json::to_string(&data);

    match post(config::url("send"), json) {
        Ok(response) => response.status().as_u16(),
        Err(_) => 0,
    }
}
