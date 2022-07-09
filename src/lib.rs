///! Atomio - parse molecular formula
///! 
///! `atomio` uses [pest.rs](https://pest.rs/) to parse molecular formulae like CH3, CuSO4.5H20 and
///! more.
///!
///! ## Example:
///! ```rs
///! use atomio::{parser::Parsed,element_group::Unit};
///! 
///! let parsed = Parsed::from_str("CH4");
///! assert!(parsed.root_group, Unit::ElementGroup {count:1, units: vec![
///!     Unit::Element { count: 1, symbol: "C".into() },
///!     Unit::Element { count: 4, symbol: "H".into() }
///! ]})
///!
///! ```

#[deny(missing_docs)]

///! Module with `Unit` enum and related functions.
pub mod unit;
pub mod parser;

use crate::parser::Parsed;
use std::io;

pub fn run() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let root: Parsed = input.trim().try_into().unwrap();
    println!("{:#?}", root.root_unit);
    println!("Composition: {:#?} ", root.root_unit.get_composition());
}
