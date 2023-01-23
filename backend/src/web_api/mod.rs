//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path, where relevant.
//

mod articles;
pub use articles::*;

pub mod extractors;

mod responses;
pub use responses::*;

mod users;
pub use users::*;
