//! If you need a copy of the CHM compiler, you can find one at:  
//! <https://github.com/EWSoftware/SHFB/blob/master/ThirdPartyTools/htmlhelp.exe>
//! 
//! Usage: install the binary, and include this `[output.chm]` in your `book.toml`
//! 
//! These options are supported:
//! - `language_code`: One of [`crate::chm::config::ChmLanguage`]. Default is `en-us`
//! - `output_path`: filename for the result. Default is `book.chm`
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

pub mod chm;
pub mod mdbook;
