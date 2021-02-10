
// --------------------------

pub trait Animal {
    fn noise(&self) -> String;
    fn age(&self) -> u8;
    fn name(&self) -> String;
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
    fn name(&self) -> String {
        let mut result: String = String::new();
        result.push_str("name: ");
        result.push_str(&self.name);
        return result;
    }
}

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
    fn name(&self) -> String {
        let mut result: String = String::new();
        result.push_str("name: ");
        result.push_str(&self.name);
        return result;
    }
}

// -------------------------------

/*
pub fn oldest(x: &impl Animal, y: &impl Animal) -> &dyn Animal
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
