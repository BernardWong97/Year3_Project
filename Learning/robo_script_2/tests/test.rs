extern crate robo_script_2 as robo;

use std::collections::BTreeSet;
use robo::pos::{Pos};

#[test]
fn some_test(){

    let smt = ['*'; 5];

    println!("{:?}", smt);



    assert_eq!(true, true);
}

#[test]
fn test_iterators(){
    let mut data_in_some_strct:BTreeSet<Pos> = BTreeSet::new();

    data_in_some_strct.insert(Pos::of(1,1));
    data_in_some_strct.insert(Pos::of(2,2));
    data_in_some_strct.insert(Pos::of(3,3));
    data_in_some_strct.insert(Pos::of(4,4));

    let set = data_in_some_strct;   // works
//    let set = &data_in_some_strct;  // doesn't work, How to adjust code to make it work??


    data_in_some_strct = set.into_iter()
        .map(|mut p| p.offset(&Pos::of(1,0)))
        .inspect(|p| println!("{:?}", *p))
        .collect();

    assert_eq!(data_in_some_strct.contains(&Pos::of(2,1)), true);
    assert_eq!(data_in_some_strct.contains(&Pos::of(3,2)), true);
    assert_eq!(data_in_some_strct.contains(&Pos::of(4,3)), true);
    assert_eq!(data_in_some_strct.contains(&Pos::of(5,4)), true);
}