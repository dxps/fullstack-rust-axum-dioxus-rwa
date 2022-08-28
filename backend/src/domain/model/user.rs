/// The main representation of the User, containing most (except password) of the details of a user.
#[derive(Debug)]
pub struct User {
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}
