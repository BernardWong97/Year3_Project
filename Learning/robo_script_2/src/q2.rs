pub mod pos {
    use std::cmp::{Ordering, PartialEq};

    #[derive(PartialOrd, PartialEq, Eq, Hash, Debug, Copy, Clone)]
    pub struct Pos {
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
}




mod test {
    use std::collections::BTreeSet;
    use create::pos::Pos;

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
        assert_eq!(data_in_some_strct.contains(&Pos::of(5,4)), false);
    }
}