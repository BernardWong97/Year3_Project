use crate::tracker::Tracker;
use std::ops::AddAssign;
use crate::pos::Pos;
use std::collections::BTreeSet;

use regex::{Regex, Captures};
#[macro_use] extern crate lazy_static;

pub mod pos;        // make submodule public
pub mod tracker;    // make submodule public

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        *self = match self {
            Direction::Right => match other
                {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    _ => Direction::Right,
                },
            Direction::Left => match other
                {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    _ => Direction::Left,
                },
            Direction::Up => match other
                {
                    Direction::Right => Direction::Right,
                    Direction::Left => Direction::Left,
                    _ => Direction::Up,
                },
            Direction::Down => match other
                {
                    Direction::Right => Direction::Left,
                    Direction::Left => Direction::Right,
                    _ => Direction::Down,
                },
        }
    }
}

type Command = (char, usize);

struct Robot {
    pub tracker: Tracker,
    pub direction: Direction,
}

#[allow(dead_code)]
impl Robot {
    pub fn new() -> Self {
        let mut tracker = Tracker::new();
        tracker.add_pos(Pos::new());

        Self {
            tracker,
            direction: Direction::Right,
        }
    }

    pub fn turn(&mut self, direction:char) -> &mut Self{
        match direction {
            'L' => self.direction += Direction::Left,
            'R' => self.direction += Direction::Right,
            _ => println!("Can't go to this direction: {}", direction),
        }

        self
    }

    pub fn drive(&mut self, units:&usize) -> &mut Self{
        let v = match self.direction {
            Direction::Right => Pos::of(1,0),
            Direction::Left => Pos::of(-1,0),
            Direction::Up => Pos::of(0,1),
            Direction::Down => Pos::of(0,-1),
        };

        let mut last = self.tracker.last;
        (0..*units)
            .map(|_| last.offset(&v).clone())
            .for_each(|pos| { self.tracker.add_pos(pos); });

        self
    }

}// impl Robot



pub fn execute(code: &str) -> String {
    // Implement your RS1 interpreter here
    let mut robot = Robot::new();

//    lazy_static! {
//        static ref RE:Regex = Regex::new(r"F+[\d]*|L+|R+").unwrap();
//    }

    let RE = Regex::new(r"F+[\d]*|L+|R+").unwrap();

    RE.captures_iter(code)
        .for_each(|it|{
            println!()
        });





    robot.tracker.to_string()
}

#[test]
fn test_execute() {
    let codes = [
        ("", "*"),
        ("FFFFF", "******"),
        ("LFFFFFRFFFRFFFRFFFFFFF", "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******"),
        ("LF5RF3RF3RF7", "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   "),
    ];



    codes.iter()
        .for_each(|&(code, result)| {
//            assert_eq!(execute(code), result);
            let mut robot = Robot::new();

            if code.len() < 1 { }
            else {
                if &code[..1] == "L" || &code[..1] == "R" {
                    code.chars().for_each(|ch| { robot.turn(ch); });
                } else if &code[..1] == "F" {
                    if code.len() > 1 {
                        if code.chars().next().unwrap().is_digit(10) {
                            let steps = code[1..].parse::<usize>().unwrap();
                            robot.drive(&steps);
                        }
                    }
                }
            }

            println!("{}", robot.tracker);


    });

    assert_eq!(true, false);
}



#[test]
fn pre_processor() {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"F+[\d]*|L+|R+").unwrap();
    }

    let codes = [
        ("", "*"),
        ("FFFFF", "******"),
        ("LFFFFFRFFFRFFFRFFFFFFF", "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******"),
        ("LF5RF3RF3RF7", "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   "),
    ];

    codes.iter()
        .for_each(|&(a, b)|{
            RE.captures_iter(a)
                .inspect(|it| println!("{:?}", it))

                .for_each(|it| println!("{:?}", &it[0]));

            println!();
        });

    assert_eq!(true, false);
}



#[cfg(test)]
mod tests {
    use crate::Robot;
    use crate::Direction;
    use crate::pos::Pos;

    #[test]
    fn test_turn(){
        let mut robot = Robot::new();
        assert_eq!(robot.direction, Direction::Right);

        robot.turn('L');
        assert_eq!(robot.direction, Direction::Up);

        robot.turn('L');
        robot.turn('L');
        robot.turn('R');
        robot.turn('L');
        robot.turn('F');
        assert_eq!(robot.direction, Direction::Down);
    }

    #[test]
    fn test_drive(){
        // test initial
        let mut robot = Robot::new();
        assert_eq!(robot.tracker.size(), 1);
        assert_eq!(robot.tracker.get_data().contains(&Pos::of(0,0)), true);

        // test drive
        let steps_right = 5usize;
        robot.drive(&steps_right);
        let mut elements = (0..steps_right)
            .map(|x| Pos::of(x as i32, 0))
            .filter(|pos| robot.tracker.get_data().contains(&pos))
            .count();

        println!("{:#?}", robot.tracker.get_data());
        assert_eq!(robot.tracker.size(), steps_right +1usize);
        assert_eq!(elements, steps_right);

        // test turn and drive
        let steps_down = 3;
        robot.turn('R')     // down
            .drive(&steps_down);     // 3 steps

        elements = (0..steps_down)
            .map(|x| Pos::of(x as i32, 0))
            .filter(|pos| robot.tracker.get_data().contains(&pos))
            .count();

        println!("{:#?}", robot.tracker.get_data());
        assert_eq!(robot.tracker.size(), steps_right +1usize +steps_down );
        assert_eq!(elements, steps_down)
    }
}