#![crate_name = "ar_cuil_cuit_validator"]

//! # ar_cuil_cuit_validator
//!
//! `cuil_cuit` is a command line utility and a small library to validate CUIL/CUIT
//! 
//! ## Installation
//! 
//! Install with cargo:
//!
//! ```toml
//! [dependencies]
//! ar_cuil_cuit_validator = "*"
//! ```
//!
//! Then use it in your code
//!
//! ```rust
//! use ar_cuil_cuit_validator::cuil_cuit;
//!
//! let valid_cuil_cuit = 27280335148;
//! let result = is_valid(valid_cuil_cuit).unwrap();
//! assert_eq!(true, result);
//! ```

pub mod cuil_cuit;
