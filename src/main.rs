mod client;
mod lib;
use ct_rs::dotenv::{parse_env};
use ct_rs::dotenv::{EnvironmentVariable, EnvironmentVariableCollection};


struct Environment<'a> {
    project_key: &'a EnvironmentVariable,
    client_id: &'a EnvironmentVariable,
    client_secret: &'a EnvironmentVariable,
    auth_url: &'a EnvironmentVariable,
    api_url: &'a EnvironmentVariable,
    scopes: &'a EnvironmentVariable,
}
struct Token {
    access_token: String
}
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
    ).await;

    let text= token.unwrap().text().await.unwrap();

    let parsedToken = json::parse(&text).unwrap();
    let ptoken = Token::from(parsedToken.into());
    print!("Token: {:?}", parsedToken);
}
