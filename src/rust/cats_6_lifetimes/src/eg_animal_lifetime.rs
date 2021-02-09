
// use std::fmt;

// --------------------------

pub trait Animal {
    fn noise(&self) -> String;
    fn age(&self) -> u8;
    fn to_string(&self) -> String;
}

// --------------------------

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Animal for Cat {
    fn noise(&self) -> String {
        let mut result: String = String::new();
        result.push_str(&self.name);
        result.push_str(" says MEOW");
        return result;
    }
    fn age(&self) -> u8 {
        self.age
    }
    fn to_string(&self) -> String {
        let mut result: String = String::new();
        result.push_str("name: ");
        result.push_str(&self.name);
        return result;
    }
}

impl Cat {
    pub fn new() -> Cat {
        Cat {
            name: String::from(""),
            age: 0,
        }
    }
}

/*
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {} age: {}", self.name, self.age)
    }
}
*/

// -------------------------------

#[derive(Debug)]
pub struct Dog {
    pub name: String,
    pub age: u8,
}

impl Animal for Dog {
    fn noise(&self) -> String {
        let mut result: String = String::new();
        result.push_str(&self.name);
        result.push_str(" says WOOF");
        return result;
    }
    fn age(&self) -> u8 {
        self.age
    }
    fn to_string(&self) -> String {
        let mut result: String = String::new();
        result.push_str("name: ");
        result.push_str(&self.name);
        return result;
    }
}

impl Dog {
    pub fn new() -> Dog {
        Dog {
            name: String::from(""),
            age: 0,
        }
    }
}

// -------------------------------

/*
pub fn oldest(x: &impl Animal, y: &impl Animal,) -> &dyn Animal
{
    println!("x: {}", x.noise());
    println!("y: {}", y.noise());
    if x.age() > y.age() {
        x
    } else {
        y
    }
}
*/

// ----------------


/*
*/
pub fn oldest<'a>(x: &'a impl Animal, y: &'a impl Animal) -> &'a dyn Animal
{
    println!("x: {}", x.noise());
    println!("y: {}", y.noise());
    if x.age() > y.age() {
        x
    } else {
        y
    }
}