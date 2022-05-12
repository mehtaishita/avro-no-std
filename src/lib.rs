
#![cfg_attr(not(feature = "std"), no_std)]

pub mod avro;
pub mod avro_validation;
#[cfg(test)]
mod avro_tests;
pub mod types;
