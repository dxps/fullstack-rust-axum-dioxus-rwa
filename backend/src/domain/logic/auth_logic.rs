use crate::{domain::model::User, AppError};

// Not used, since currently async trait methods are not supported.
// Thus, `UserRepoPg` could not implement this while need
pub trait UserRepo {
    fn save(&self, user: &User, password: String) -> Result<(), AppError>;
}
