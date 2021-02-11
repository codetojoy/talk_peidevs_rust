
use std::vec::Vec;

use rayon::prelude::*;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

// log with thread id prefix
pub fn t_log(s: &str) {
    println!("TRACER {:?} -- {}", std::thread::current().id(), s);
}

fn serial_emitter(cats: &Vec<Cat>) {
    cats.into_iter().for_each(|cat|
        t_log(&format!("{:?}", cat))
    );
}

fn parallel_emitter(cats: &Vec<Cat>) {
    cats.into_par_iter().for_each(|cat|
        t_log(&format!("{:?}", cat))
    );
}

fn serial_filter(cats: &Vec<Cat>, target_age: u8) -> Vec<&Cat> {
    let result = cats.into_iter().filter(|&cat|
        cat.age > target_age
    ).collect();
    return result;
}

fn parallel_filter(cats: &Vec<Cat>, target_age: u8) -> Vec<&Cat> {
    let result = cats.into_par_iter().filter(|&cat|
        cat.age > target_age
    ).collect();
    return result;
}

fn main() {
    let mut cats = vec![];
    cats.push(Cat{name: String::from("Bach"), age: 1});
    cats.push(Cat{name: String::from("Beethoven"), age: 2});
    cats.push(Cat{name: String::from("Mozart"), age: 3});
    cats.push(Cat{name: String::from("Schubert"), age: 4});
    cats.push(Cat{name: String::from("Wagner"), age: 1});
    cats.push(Cat{name: String::from("Vivaldi"), age: 2});
    cats.push(Cat{name: String::from("Brahms"), age: 3});
    cats.push(Cat{name: String::from("Chopin"), age: 4});
    cats.push(Cat{name: String::from("Stravinsky"), age: 1});
    cats.push(Cat{name: String::from("Shostakovich"), age: 2});

    let args: Vec<String> = std::env::args().collect();
    let which = args[1].parse::<u8>().unwrap();
    println!("TRACER running case: {}", which);

    if which == 1 {
       serial_emitter(&cats);
    } else if which == 2 {
       parallel_emitter(&cats);
    } else if which == 3 {
        let age = 2;
        let results = serial_filter(&cats, age);

        for result in results.iter() {
            println!("TRACER {:?}", result);
        }
    } else if which == 4 {
        let age = 2;
        let results = parallel_filter(&cats, age);

        for result in results.iter() {
            println!("TRACER {:?}", result);
        }
    }

    println!("Ready.");
}
