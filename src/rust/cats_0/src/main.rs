
use std::vec::Vec;

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn main() {
    let mut cats = Vec::<Cat>::new();
    let cat1 = Cat {name: String::from("Mozart"), age: 4};
    let cat2 = Cat {name: String::from("Chopin"), age: 3};
    let cat3 = Cat {name: String::from("Bach"), age: 5};
    cats.push(cat1);
    cats.push(cat2);
    cats.push(cat3);

    for cat in cats {
        println!("TRACER {:?}", cat);
    }
    println!("TRACER Ready.");
}
