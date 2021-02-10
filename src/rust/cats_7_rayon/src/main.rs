
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

fn serial_emitter(list: &Vec<Cat>) {
    list.into_iter().for_each(|info|
        t_log(&format!("{:?}", info))
    );
}

fn parallel_emitter(list: &Vec<Cat>) {
    list.into_par_iter().for_each(|info|
        t_log(&format!("{:?}", info))
    );
}

fn serial_filter(list: &Vec<Cat>, threshold: u8) -> Vec<&Cat> {
    let result = list.into_iter().filter(|&info|
        info.age > threshold
    ).collect();
    return result;
}

fn parallel_filter(list: &Vec<Cat>, threshold: u8) -> Vec<&Cat> {
    let result = list.into_par_iter().filter(|&info|
        info.age > threshold
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

    let which = 2;

    if which == 1 {
       serial_emitter(&cats);
    } else if which == 2 {
       parallel_emitter(&cats);
    } else if which == 3 {
        let age = 3;
        let results = serial_filter(&cats, age);

        for result in results.iter() {
            println!("TRACER {:?}", result);
        }
    } else if which == 4 {
        let age = 3;
        let results = parallel_filter(&cats, age);

        for result in results.iter() {
            println!("TRACER {:?}", result);
        }
    }

    println!("Ready.");
}
