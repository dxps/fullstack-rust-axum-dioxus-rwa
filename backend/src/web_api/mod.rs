//
// Registering the modules in the module tree and
// re-exporting their entries with a shorter path.
//

mod responses;
pub use responses::*;

mod extractors;
pub use extractors::*;

mod users;
pub use users::*;

// Articles mgmt

mod articles;
pub use articles::*;
