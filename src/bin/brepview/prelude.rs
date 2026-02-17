#![allow(unused_imports)]
pub(crate) use anyhow::Result;
pub(crate) type DynRes<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) use log::{debug, error, info, trace, warn};
