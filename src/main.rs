// This is how we comment in Rust
use std::fmt::{self};

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hi my name is {}, I'm {} years old", self.name, self.age)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    let point = Point2D{x: 3.3, y: 7.2};

    println!("Comaparing structures");
    println!("{:#?}", peter);
    println!("{}", peter);

    println!("Comparing Points.");
    println!("{}", point);
    println!("{:?}", point)
}
