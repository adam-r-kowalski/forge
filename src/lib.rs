#![no_std]
#![forbid(unsafe_code)]
#![feature(ip_in_core)]
#![feature(error_in_core)]
#![feature(iter_array_chunks)]

pub mod array;
pub mod core;
pub mod effect;
mod evaluator;
pub mod expression;
pub mod extract;
pub mod html;
pub mod map;
mod numerics;
mod parser;
mod peeking_take_while;
pub mod server;
pub mod sql;
mod tokenizer;

pub use evaluator::{evaluate, evaluate_expressions, evaluate_source, pattern_match};
pub use expression::{Environment, Expression};
pub use numerics::{bits_to_decimal_digits, decimal_digits_to_bits, ratio, Float};
pub use parser::parse;
pub use peeking_take_while::PeekableExt;
pub use tokenizer::{Token, Tokens};
