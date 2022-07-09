use std:: collections::HashMap;

/// Represents a unit. It can be either a Element (with a count and symbol) or a ElementGroup 
#[derive(Clone,Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// Represents a group of elements with a count and a array of `Unit`s.
    ElementGroup {
        /// How many of this ElementGroup is present
        count: u32,
        /// Array of units present inside.
        units: Vec<Unit>
    },
    /// Represents a element with a count and symbol.
    Element {
        /// How many of this Element is present
        count: u32,
        /// Symbol of the element
        symbol: String
    }
}

impl Unit {
    /// Returns a `HashMap` with keys being the symbol of the element and values being the number
    /// of those atoms.
    pub fn get_composition(&self) -> HashMap<&String, u32> {
        let mut map = HashMap::new();

        match self {
            Self::Element {count,symbol} => {
                let v = map.entry(symbol).or_insert(0);
                *v += count;
            },
            Self::ElementGroup {count,units} => {
                for unit in units {
                    let new_map = unit.get_composition();

                    for (k,_v) in new_map {
                        let v = map.entry(k).or_insert(0);
                        *v += _v * count
                    }
                }
            }
        }

        map
    }
}
