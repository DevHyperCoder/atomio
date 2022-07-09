# ATOMIO - Crate to parse molecular formula

Parse chemical formula like `CH4`, `Al2(SO4)3`, `K2Cr2O7` etc.

Usage is simple:

```rs
use atomio::{parser::Parsed,element_group::Unit};

let parsed = Parsed::from_str("CH4");
assert!(parsed.root_group, Unit::ElementGroup {count:1, units: vec![
    Unit::Element { count: 1, symbol: "C".into() },
    Unit::Element { count: 4, symbol: "H".into() }
]})
```

## Contributions

Pull Requests and Issues are accepted.

## LICENSE

`atomio` is licensed under the MIT License. Our copy of MIT License can be found [here](./LICENSE)
