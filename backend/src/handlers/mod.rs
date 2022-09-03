// Registering the modules in the module tree and
// re-exporting their entries with a shorter path.

mod common_payloads;
pub use common_payloads::*;

mod common_responses;
pub use common_responses::*;

mod register_user;
pub use register_user::*;

mod login_user;
pub use login_user::*;

mod get_curr_user;
pub use get_curr_user::*;

mod token_claims_extractor;
pub use token_claims_extractor::*;
