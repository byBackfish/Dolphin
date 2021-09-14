extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate nbt;

pub mod objects;
pub mod util;

pub use objects::*;
pub use util::*;

pub use util::redutil::Connection as Connection;

use std::error::Error;
use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Box<dyn Error>>;
