use serde::Serialize;

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
