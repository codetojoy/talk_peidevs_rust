
use std::fmt;

pub struct Person {
    pub name: String,
    pub birth_year: u32
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {} born: {}", self.name, self.birth_year)
    }
}

impl Person {
    pub fn new() -> Person {
        Person {
            name: String::from(""),
            birth_year: 0,
        }
    }
}

pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}