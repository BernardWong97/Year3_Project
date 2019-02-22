
use std::collections::BTreeSet;
use crate::pos::Pos;
use std::fmt;
use std::mem;


#[derive(Debug)]
pub struct Tracker {
    pub min: Pos,
    pub max: Pos,
    pub last: Pos,
    data: BTreeSet<Pos>,
}

#[allow(dead_code)]
impl Tracker {
    pub fn new() -> Self {
        Self {
            min: Pos::new(),
            max: Pos::new(),
            last: Pos::new(),
            data: BTreeSet::new(),
        }
    }

    pub fn add_pos(&mut self, pos:Pos) -> &mut Self {
        // update min limits
        if self.min.x > pos.x { self.min.x = pos.x; }
        if self.min.y > pos.y { self.min.y = pos.y; }
        // update max limits
        if self.max.x < pos.x { self.max.x = pos.x; }
        if self.max.y < pos.y { self.max.y = pos.y; }

        self.last.set_to(pos.x, pos.y);
        &self.data.insert(pos);

        self
    }

    pub fn update_all(&mut self, update_by:&Pos) -> &mut Self{
        let set:BTreeSet<Pos> = mem::replace(&mut self.data, BTreeSet::new());

        // update fields
        self.min.offset(&update_by);
        self.max.offset(&update_by);
        self.last.offset(&update_by);

        // update all data
        self.data = set.into_iter()
            .map(|mut p| p.offset(update_by))
//            .inspect(|p| println!("{:?}", *p))
            .collect();

        self
    }



    pub fn get_data(& self) ->  &BTreeSet<Pos> {
        &self.data
    }

    pub fn last_pos(&self) -> &Pos {
        &self.last
    }

    pub fn max_pos(&self) -> &Pos {
        &self.max
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Tracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut res = String::new();
        let mut curr = Pos::new();

        for x in self.min.x..self.max.x {
            for y in self.min.y..self.max.y {
                curr.set_to(x, y);
                if self.data.contains(&curr) {
                    res.push('*');
                }
                else{
                    res.push(' ');
                }
            }
            res.push('\r');
            res.push('\n');
        }

        write!(f, "{}",res)
    }
}




#[cfg(test)]
mod tests {
    use crate::pos::Pos;
    use crate::tracker::Tracker;
//    use std::mem;

    #[test]
    fn test_obj_len() {
        let mut obj = Tracker::new();
        obj.add_pos(Pos::new())
            .add_pos(Pos::of(1,1))
            .add_pos(Pos::of(5,1))
            .add_pos(Pos::of(2,2))
            .add_pos(Pos::of(2,2))
            .add_pos(Pos::of(0,0));

        println!("{:#?}", obj);
        assert_eq!(obj.size(), 4);
        assert_eq!(*obj.last_pos(), Pos::of(0, 0));
        assert_eq!(*obj.max_pos(), Pos::of(5,2));

    }

    #[test]
    fn test_min_max() {
        let mut obj = Tracker::new();
        obj.add_pos(Pos::new())
            .add_pos(Pos::of(1,5))
            .add_pos(Pos::of(-5,-10))
            .add_pos(Pos::of(2,-11))
            .add_pos(Pos::of(2,2));

        assert_eq!(obj.max, Pos::of(2,5));
        assert_eq!(obj.min, Pos::of(-5,-11));
        assert_eq!(obj.last, Pos::of(2,2));
        assert_eq!(obj.get_data().len(), 5);
    }

    #[test]
    fn test_update_all() {
        let mut obj = Tracker::new();
        obj.add_pos(Pos::new())
            .add_pos(Pos::of(1,1))
            .add_pos(Pos::of(-5,-10))
            .add_pos(Pos::of(2,2))
            .add_pos(Pos::of(2,2));

        println!("Before update: {:#?}", obj.get_data());
        println!("min: {:#?}", obj.min);

        obj.update_all(&Pos::of(1, 1));

        println!("After update: {:#?}", obj.get_data());
        println!("min: {:#?}", obj.min);

        assert_eq!(obj.get_data().contains(&Pos::of(3,3)), true);
        assert_eq!(obj.get_data().contains(&Pos::of(-4,-9)), true);
//        assert_eq!(obj.max, Pos::of(3,3));
        assert_eq!(obj.min, Pos::of(-4,-9));
        assert_eq!(obj.last, Pos::of(3,3));
    }

}