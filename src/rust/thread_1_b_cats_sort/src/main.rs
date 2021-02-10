
use std::vec::Vec;
use std::thread;

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

    // `move` is crucial for this to work
    let handle = thread::spawn(move || {
        cats.sort_by(|a, b| a.name.cmp(&b.name));

        for cat in cats {
            println!("TRACER {:?}", cat);
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