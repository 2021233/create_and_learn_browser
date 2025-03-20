#![no_std]

extern crate alloc;

pub mod error;
pub mod http;
pub mod url;

use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedInput(String),
    InvalidUI(String),
    Orther(String),
}
