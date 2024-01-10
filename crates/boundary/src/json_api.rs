use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestPasswordReset {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfirmEmailAddress {
    pub token: String,
}
