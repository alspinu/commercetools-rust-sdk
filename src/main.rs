mod client;
mod dotenv;

use crate::dotenv::{
    dotenv::{parse_env, EnvironmentVariableCollection},
    types::{Environment,Token},
};

#[tokio::main]
async fn main() {
    let parsed_env = parse_env();

    let client = Environment {
        project_key: parsed_env.getByKey("CTP_PROJECT_KEY"),
        client_id: parsed_env.getByKey("CTP_CLIENT_ID"),
        client_secret: parsed_env.getByKey("CTP_CLIENT_SECRET"),
        auth_url: parsed_env.getByKey("CTP_AUTH_URL"),
        api_url: parsed_env.getByKey("CTP_API_URL"),
        scopes: parsed_env.getByKey("CTP_SCOPES"),
    };

    let token = client::auth::authenticate(
        client.client_id.value.as_str(),
        client.client_secret.value.as_str(),
    )
    .await;

    let text = token.unwrap().text().await;

    let parsedToken = json::parse(&text).unwrap();
    print!("Token: {:?}", Token::from(parsedToken));
}
