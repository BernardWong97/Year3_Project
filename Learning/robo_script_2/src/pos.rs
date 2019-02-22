
use std::cmp::{Ordering, PartialEq};

#[derive(PartialOrd, PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Pos {

    pub fn new() -> Self {
        Self {
            x: 0i32, y: 0i32
        }
    }

    pub fn of(x:i32, y:i32) -> Self {
        Self {
            x, y,
        }
    }

    pub fn offset(&mut self, pos:&Self) -> Self {
        self.x += pos.x;
        self.y += pos.y;

        *self
    }

//    pub fn set_to(&mut self, pos:&Self) -> Self {
//        self.x = pos.x;
//        self.y = pos.y;
//
//        *self
//    }

    pub fn set_to(&mut self, x:i32, y:i32) -> Self {
        self.x = x;
        self.y = y;

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

#[cfg(test)]
mod tests {
    use crate::pos::Pos;

    #[test]
    fn test_pos() {
        let mut pos = Pos::of(1, 1);
        pos.offset(&Pos::of(1, 1));
        let pos1 = pos.offset(&Pos::of(1, 1)).clone();
        let mut pos2 = Pos::new();
        pos2.clone_from(&pos);
        pos2.offset(
            &Pos::of(1, 1)
                .offset(&Pos::of(1, 1)
                ))
            .offset(&Pos::of(1, 1));


        assert_eq!(pos, Pos::of(3,3));
        assert_eq!(pos1, Pos::of(3,3));
        assert_eq!(pos2, Pos::of(5,5));
        assert_eq!(pos, Pos::of(3,3));

    }

    #[test]
    fn test_offset() {
        assert_eq!(Pos::of(2, 2), Pos::new().offset(&Pos::of(2,2)));
        assert_eq!(Pos::of(4, 4),
                   Pos::new()
                       .offset(&Pos::of(2,2))
                       .offset(&Pos::of(2,2)));
    }

    #[test]
    fn test_comparison(){
        // equality
        assert_eq!(true, Pos::of(1, 2) == Pos::of(1, 2));
        assert_eq!(true, Pos::of(-1, 2) == Pos::of(-1, 2));
        assert_eq!(false, Pos::of(-1, 2) == Pos::of(-1, -2));
        assert_eq!(true, Pos::of(-1, 2) == Pos::new().offset(&Pos::of(-1, 2)));
        assert_eq!(true, Pos::of(-1, 2) == Pos::of(1, -2).offset(&Pos::of(-2, 4)));
        assert_eq!(false, Pos::of(-1, 2) != Pos::of(1, -2).offset(&Pos::of(-2, 4)));
        // comparison
        assert_eq!(true, Pos::of(-1, 2) > Pos::of(-1, -2));
        assert_eq!(true, Pos::of(-1, 2) > Pos::of(-2, 2));
        assert_eq!(true, Pos::of(-1, -3) < Pos::of(-1, -2));
        assert_eq!(true, Pos::of(-1, 2) < Pos::of(1, -2));
    }

    #[test]
    fn test_setter(){
        let p = Pos::of(2, 3);
        let mut p1 = Pos::new();
        assert_ne!(p, p1);

        p1.set_to(p.x, p.y);
        assert_eq!(p1, p);
    }
}