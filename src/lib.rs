#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]


#[macro_use]
pub mod controller;
pub mod config;
pub mod service;
mod domain;
pub mod util;
mod dao;
pub mod middleware;