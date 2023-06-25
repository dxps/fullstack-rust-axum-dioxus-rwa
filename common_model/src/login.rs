use serde::{Deserialize, Serialize};

/// This is returned in case of a successful user registration or authentication.<br/>
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulLoginDTO {
    pub user: UserDTO,
}

/// In the payload, this must be the value of the "user" attribute<br/>
/// aka using such return `(StatusCode::OK, Json(json!({ "user": dto })))`
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub email: String,
    pub token: Option<String>,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}
