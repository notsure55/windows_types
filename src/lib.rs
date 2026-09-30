#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![feature(sized_hierarchy)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

#[cfg(feature = "kernel")]
pub mod kernel;

#[cfg(feature = "user")]
pub mod user;
