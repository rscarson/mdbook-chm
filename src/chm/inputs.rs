//! All the included file parsers
//!
//! Each will load a type of file and render it as an HTML document
//! 
//! Currently only markdown is supported

mod md;
pub use md::load as md_load;