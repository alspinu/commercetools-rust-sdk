use serde_derive::Deserialize;
use serde_derive::Serialize;

pub struct Environment<'a> {
    pub project_key: &'a EnvironmentVariable,
    pub client_id: &'a EnvironmentVariable,
    pub client_secret: &'a EnvironmentVariable,
    pub auth_url: &'a EnvironmentVariable,
    pub api_url: &'a EnvironmentVariable,
    pub scopes: &'a EnvironmentVariable,
}
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
}
