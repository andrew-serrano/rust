// Don't warn us
#![allow(dead_code)]

// Import
use std::fmt;

// Drive Debug to allow debugging square trait
#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}

// Methods
impl Square {
    fn area (&self) -> u32 {
        return self.width * self.height;
    }
}

// Associated Functions
impl Square {
    fn new (width: u32, height: u32) -> Self {
        return Self {
            width: width,
            height: height,
        };
    }
}

// Implement the display trait for square to allow printing
impl fmt::Display for Square {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,"({}, {})", self.width, self.height);
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    RightKey(String),
    LeftKey(String),
}

impl Direction {
    fn match_direction (&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Up")),
            Direction::Down(_) => Keys::DownKey(String::from("Up")),
            Direction::Right(_) => Keys::RightKey(String::from("Up")),
            Direction::Left(_) => Keys::LeftKey(String::from("Up")),
        }
    }
}

impl Keys {
    fn destruct (&self) -> &String {
        match self {
            Keys::UpKey(ref position) => position,
            Keys::DownKey(ref position) => position,
            Keys::RightKey(ref position) => position,
            Keys::LeftKey(ref position) => position,
        }
    }
}

fn main() {
    let square = Square {
        width: 52,
        height: 52,
    };

    let new_square = Square::new(100,100);

    let test = 5;

    match test {
        // Binds 5 to N if expression is true @ binding syntax or operator?
        n @ 1..=5 => println!("{}", n),
        _ => println!("Nothing"),
    }

    let up = Direction::Up(Point {
        x: 1,
        y:1, 
    });

    println!("{}", square.area());
    println!("{}", new_square.area());
    println!("Direction: {:#?}", up.match_direction());
    println!("Key: {:#?}", up.match_direction().destruct());
}