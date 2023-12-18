//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path, where relevant.
//

pub mod extractors;

mod routes;
pub use routes::*;

mod responses;
pub use responses::*;

mod articles;
pub use articles::*;

mod health;
pub use health::*;

mod users;
pub use users::*;

mod token;
