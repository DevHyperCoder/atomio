use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::unit::Unit;

///! Struct to parse molecular formula using pest.rs
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MoleculeParser {}

///! Tree data structure to hold the tree of elements and thier count
#[derive(Clone,Debug,PartialEq,Eq, PartialOrd, Ord)]
pub struct Parsed {
    /// Root unit of the tree.
    pub root_unit: Unit,
}

impl TryFrom<String> for Parsed {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for Parsed {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let main = match MoleculeParser::parse(Rule::main, value) {
            Err(_) => return Err("Unable to parse chemical formula.".to_string()),
            Ok(e) => e
        };

        let count = 1;
        let mut units = vec![];

        for p in main {
            match p.as_rule() {
                Rule::EOI => {}
                _ => units.push(Parsed::get_unit_tree(p)),
            }
        }

        Ok(Parsed {root_unit: Unit::ElementGroup {
            count,
            units
        }})
    }
}


impl Parsed {
    fn get_unit_tree(p: Pair<Rule>) -> Unit {
        match p.as_rule() {
            Rule::element => {
                let mut symbol = String::new();
                let mut count = 0;

                for i in p.into_inner() {
                    match i.as_rule() {
                        Rule::count => {
                            count = i.as_span().as_str().parse::<u32>().unwrap();
                        }
                        Rule::element_name => {
                            symbol = i.as_span().as_str().to_string()
                        }
                        _ => unreachable!(),
                    }
                }

                Unit::Element{
                    symbol,
                    count
                }
            }
            Rule::group | Rule::root_group => {
                let mut count = 0;
                let mut units = vec![];

                for i in p.into_inner() {
                    match i.as_rule() {
                        Rule::count | Rule::root_count => {
                            count =  i.as_span().as_str().parse::<u32>().unwrap();
                        }
                        Rule::element | Rule::root_group | Rule::group => {
                            units.push(Parsed::get_unit_tree(i))
                        }
                        _ => unreachable!("{:?}", i),
                    }
                }

                Unit::ElementGroup {
                    count,
                    units
                }
            }
            _ => unreachable!("{:?}", p),
        }
    }
}
