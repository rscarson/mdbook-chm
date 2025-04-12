//! This module contains the CHM project writer
//! 
//! It includes a struct allowing you to easily build out a CHM project.
//! 
//! You also have access to the underlying types in the project, like [`config::hhp::ChmProject`] but I recommend against using those directly
//! 
//! Instead use [`ChmBuilder`]
mod config;
pub use config::*;

pub mod inputs;
pub mod utilities;
