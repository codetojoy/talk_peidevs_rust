
use std::vec::Vec;
use std::fmt;
use std::thread;

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

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn main() {
    let mut cats = Vec::<Cat>::new();
    let cat1 = Cat {name: String::from("Mozart"), age: 4, .. Cat::new()};
    let cat2 = Cat {name: String::from("Chopin"), age: 3, .. Cat::new()};
    let cat3 = Cat {name: String::from("Bach"), age: 5, .. Cat::new()};
    cats.push(cat1);
    cats.push(cat2);
    cats.push(cat3);

    // `move` is crucial for this to work
    let handle = thread::spawn(move || {
        cats.sort_by(|a, b| b.name.cmp(&a.name));

        for cat in cats {
            println!("TRACER {}", cat);
        }
    });

    handle.join().unwrap();

    // we can't use `cats` because ownership was moved
    /*
    for cat in cats {
        println!("TRACER {}", cat);
    }
    */
    println!("TRACER Ready.");
}