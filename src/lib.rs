extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate nbt;


pub mod websocket;

pub mod objects;
pub mod util;

pub use objects::*;
pub use util::*;

use std::error::Error;
use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Box<dyn Error>>;
