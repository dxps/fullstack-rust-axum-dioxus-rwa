//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path, where relevant.
//

mod responses;
pub use responses::*;

pub mod extractors;

mod users;
pub use users::*;

mod articles;
pub use articles::*;
