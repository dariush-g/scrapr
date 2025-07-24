pub mod protocols;
pub mod request;
pub mod url;

pub mod prelude {
    pub use crate::protocols::*;
    pub use crate::request::*;
    pub use crate::url::*;
    pub use pyo3::prelude::*;
}

// pub mod prelude {
//     pub use crate::protocols::*;
//     pub use crate::request::*;
//     pub use crate::url::*;
// }
