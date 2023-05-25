use serde::{Deserialize, Serialize};

/// This is returned in case of a successful user registration or authentication.<br/>
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulLoginDTO {
    pub user: SuccessfulLoginUserDTO,
}

/// In the payload, this must be the value of the "user" attribute.<br/>
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulLoginUserDTO {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}
