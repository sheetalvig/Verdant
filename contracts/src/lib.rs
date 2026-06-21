#![no_std]

// AUTO-GENERATED module composition
mod modules {
    pub mod marketplace;
    pub mod token;
}

pub use modules::marketplace::*;
pub use modules::token::*;

// shared platform constants
pub const FEE_BPS: i128 = 100;
pub const MIN_DEPOSIT: i128 = 1000000;
pub const GRACE_SECS: i128 = 604800;
