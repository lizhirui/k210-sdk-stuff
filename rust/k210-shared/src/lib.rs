#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]

pub mod board;
#[cfg(not(test))]
pub mod panic;
pub mod soc;
mod util;
