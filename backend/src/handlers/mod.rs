//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path.
//

mod dtos;
pub use dtos::*;

mod responses;
pub use responses::*;

mod extractors;
pub use extractors::*;

mod register_user;
pub use register_user::*;

mod login_user;
pub use login_user::*;

mod get_curr_user;
pub use get_curr_user::*;

mod update_curr_user;
pub use update_curr_user::*;

mod get_user_profile;
pub use get_user_profile::*;

mod follow_user;
pub use follow_user::*;

// Articles mgmt

mod articles;
pub use articles::*;
