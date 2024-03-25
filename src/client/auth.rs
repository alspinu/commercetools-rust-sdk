use std::collections::HashMap;
use reqwest::{self, Response, Client, Error};
use crate::dotenv::dotenv::{parse_env, EnvironmentVariableCollection};

pub async fn authenticate(_client_id: &str, _client_secret: &str) -> Result<Response, Error> {
    let parsedEnv = parse_env();
    let client = Client::new();
    let client_id = parsedEnv.getValueByKey("CTP_CLIENT_ID");
    let client_secret = parsedEnv.getValueByKey("CTP_CLIENT_SECRET");
    let _encoded = String::new() + client_id + ":" + &client_secret;
    let auth_url = String::new() + parsedEnv.getValueByKey("CTP_AUTH_URL") + "/oauth/token";
    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");
    form.insert("scope", "manage_project:training-exjs-20240321-shared");
    let body = client
        .post(auth_url)
        .basic_auth(client_id, Some(client_secret))
        .form(&form)
        .send().await?;
    

    return Ok(body);
}
