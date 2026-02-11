#![allow(unused)]
pub(crate) use anyhow::Result;
pub(crate) type DynRes<T> = std::result::Result<T, Box<dyn std::error::Error>>;
