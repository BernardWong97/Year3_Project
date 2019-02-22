

use std::collections::BTreeSet;
use std::cmp::{Ordering, PartialEq};

#[derive(PartialOrd, PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pos {
    pub x: i32,
    pub y:i32,
}

#[allow(dead_code)]
impl Pos {
    pub fn of(x:i32, y:i32) -> Self {
        Self { x, y, }
    }

    pub fn offset(&mut self, pos:&Self) -> Self {
        self.x += pos.x;
        self.y += pos.y;

        *self
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x { Ordering::Less }
        else if self.eq(other) { Ordering::Equal }
        else { Ordering::Greater }
    }
}

struct Tracker {
    data: BTreeSet<Pos>,
}

#[allow(dead_code)]
impl Tracker {
    pub fn new() -> Self {
        let mut data = BTreeSet::new();
        data.insert(Pos::of(0,0));

        Self { data, }
    }

    pub fn add_pos(&mut self, pos:Pos) -> &mut Self {
        &self.data.insert(pos);

        self
    }

    // Probably is a better way
    pub fn get_data(&self) -> &BTreeSet<Pos> {
        &self.data
    }
}



#[test]
fn test_iterators(){
    let mut tracker = Tracker::new();

    tracker.add_pos(Pos::of(1,1))
        .add_pos(Pos::of(1,1))
        .add_pos(Pos::of(2,2))
        .add_pos(Pos::of(3,3))
        .add_pos(Pos::of(4,4))
        .add_pos(Pos::of(4,4));

    let mut data = tracker.get_data();

    assert_eq!(data.len(), 5);

    let v = Pos::of(1,0);

    // Problematic code here
    data.into_iter()
        .map(|mut p| *p.offset(&v))
        .for_each(|p| println!("{:?}", p));
    // .collect::<BTreeSet<Pos>>();



    assert_eq!(true, false); // just to print it
}
