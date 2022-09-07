use serde::Serialize;

use crate::domain::model::UserId;

/// The payload returned in case of a successful user registration or authentication.
#[derive(Debug, Serialize)]
pub struct UserOutDTO {
    pub user: UserOutDTOUserAttrs,
}

#[derive(Debug, Serialize)]
pub struct UserOutDTOUserAttrs {
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
