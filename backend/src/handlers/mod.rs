// Registering the modules in the module tree and
// re-exporting their entries with a shorter path.

mod payloads;
pub use payloads::*;

mod responses;
pub use responses::*;

mod register_user;
pub use register_user::*;

mod login_user;
pub use login_user::*;

mod get_curr_user;
pub use get_curr_user::*;

mod update_curr_user;
pub use update_curr_user::*;

mod token_claims_extractor;
pub use token_claims_extractor::*;
