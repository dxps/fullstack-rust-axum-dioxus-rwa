use serde::Serialize;

/// The payload returned in case of a successful user registration or authentication.
#[derive(Debug, Serialize)]
pub struct UserAuthnOutputDTO {
    pub user: UserInfoDTO,
}

#[derive(Debug, Serialize)]
pub struct UserInfoDTO {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}
