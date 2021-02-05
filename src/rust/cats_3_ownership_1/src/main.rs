
use std::fmt;

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Cat {
    pub fn new() -> Cat {
        Cat {
            name: String::from(""),
            age: 0,
        }
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {} age: {}", self.name, self.age)
    }
}

fn main() {
    let cat1 = Cat {name: String::from("Mozart"), age: 4, .. Cat::new()};
    let cat2 = cat1;

    // !!!
    // println!("TRACER {}", cat1);
    println!("TRACER {}", cat2);
}
