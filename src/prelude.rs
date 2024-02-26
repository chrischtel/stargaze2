//! Crate prelude
#![allow(unused)]

pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);

pub use std::format as f;

pub use serde::Deserialize;
pub use std::io;
use crate::utils::env;