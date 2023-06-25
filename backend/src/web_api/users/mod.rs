//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path.
//

mod follow_user;
pub use follow_user::*;

mod get_curr_user;
pub use get_curr_user::*;

mod get_user_profile;
pub use get_user_profile::*;

mod login_user;
pub use login_user::*;

mod register_user;
pub use register_user::*;

mod responses;

mod update_curr_user;
pub use update_curr_user::*;
