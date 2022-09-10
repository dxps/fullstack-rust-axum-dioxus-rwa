use serde::Serialize;

use crate::domain::model::UserId;

/// This is returned in case of a successful user registration or authentication.<br/>
/// In the payload, this must be the value of the "user" attribute.<br/>
/// In other words, the response can look like this `Json(json!( "user": dto ))`.
#[derive(Debug, Serialize)]
pub struct UserOutDTO {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserProfileDTO {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: Option<Vec<UserId>>,
}
