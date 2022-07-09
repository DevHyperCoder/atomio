use atomio::{parser::Parsed,unit::Unit};

macro_rules! test {
    ($s:expr,$root_unit:expr) => {
        println!("{}",$s);
        let parsed: Parsed = $s.try_into().unwrap();
        assert_eq!(parsed, Parsed {
            root_unit: $root_unit
        })
    };
}

#[test]
fn parse_error() {
    let parsed: Result<Parsed,String> = "asdf".try_into();
    assert!(parsed.is_err());

    let parsed: Result<Parsed,String> = format!("CH4").try_into();
    assert!(parsed.is_ok())
}

#[test]
fn parse_simple() {
    test!("CH4",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 1, symbol: "C".into() },
        Unit::Element { count: 4, symbol: "H".into() },
    ]});

    test!("5H2O",Unit::ElementGroup { count: 1, units: vec![
        Unit::ElementGroup {
            count: 5,
            units: vec![
                Unit::Element { count: 2, symbol: "H".into() },
                Unit::Element { count: 1, symbol: "O".into() },
            ]
        }
    ]});

    test!("CH3OH",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 1, symbol: "C".into() },
        Unit::Element { count: 3, symbol: "H".into() },
        Unit::Element { count: 1, symbol: "O".into() },
        Unit::Element { count: 1, symbol: "H".into() },
    ]});

    test!("CuSO4", Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 1, symbol: "Cu".into() },
        Unit::Element { count: 1, symbol: "S".into()},
        Unit::Element { count: 4, symbol: "O".into()},
    ]});

    test!("Al2SO4",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 2, symbol: "Al".into() },
        Unit::Element { count: 1, symbol: "S".into()},
        Unit::Element { count: 4, symbol: "O".into()},
    ]});

    test!("C16H18ClN3S",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 16, symbol: "C".into() },
        Unit::Element { count: 18, symbol: "H".into()},
        Unit::Element { count: 1, symbol: "Cl".into()},
        Unit::Element { count: 3, symbol: "N".into()},
        Unit::Element { count: 1, symbol: "S".into()},
    ]});
}

#[test]
fn parse_group() {
    test!("Ca(OH)2",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element { count: 1, symbol: "Ca".into() },
        Unit::ElementGroup { count: 2, units: vec![
            Unit::Element { count: 1, symbol: "O".into() },
            Unit::Element { count: 1, symbol: "H".into() },
        ]}
    ]});

    test!("(NH4)2SO4",Unit::ElementGroup { count: 1, units: vec![
        Unit::ElementGroup { count: 2, units: vec![
            Unit::Element { count: 1, symbol: "N".into() },
            Unit::Element { count: 4, symbol: "H".into() },
        ]},
        Unit::Element { count: 1, symbol: "S".into()},
        Unit::Element { count: 4, symbol: "O".into()},
    ]});

    test!("Mg3(PO4)2",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element {count:3, symbol: "Mg".into()},
        Unit::ElementGroup { count: 2, units: vec![
            Unit::Element { count: 1, symbol: "P".into()},
            Unit::Element { count: 4, symbol: "O".into()},
        ]},
    ]});

    test!( "C4H4O6KNa.4H2O",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element {count:4, symbol: "C".into()},
        Unit::Element {count:4, symbol: "H".into()},
        Unit::Element {count:6, symbol: "O".into()},
        Unit::Element {count:1, symbol: "K".into()},
        Unit::Element {count:1, symbol: "Na".into()},
        Unit::ElementGroup { count: 4, units: vec![
            Unit::Element { count: 2, symbol: "H".into()},
            Unit::Element { count: 1, symbol: "O".into()},
        ]},
    ]});

    test!("Pb(C2H3O2)2",Unit::ElementGroup { count: 1, units: vec![
        Unit::Element {count:1, symbol: "Pb".into()},
        Unit::ElementGroup { count: 2, units: vec![
            Unit::Element {count:2, symbol: "C".into()},
            Unit::Element {count:3, symbol: "H".into()},
            Unit::Element {count:2, symbol: "O".into()},
        ]},
    ]});


    //test!("Na2[B4O5(OH)4]·8H2O",Unit::ElementGroup { count: 1, units: vec![
        //Unit::Element {count:2, symbol: "Na".into()},
        //Unit::ElementGroup { count: 1, units: vec![
            //Unit::Element {count:2, symbol: "C".into()},
            //Unit::Element {count:3, symbol: "H".into()},
            //Unit::Element {count:2, symbol: "O".into()},
        //]},
    //]});
}
