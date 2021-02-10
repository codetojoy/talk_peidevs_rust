
use std::thread;
use std::vec::Vec;

use rayon::prelude::*;

#[derive(Debug)]
struct Cat {
    x: u32,
    age: u8,
}

// log with thread id prefix
pub fn t_log(s: &str) {
    println!("TRACER {:?} -- {}", thread::current().id(), s);
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
    cats.push(Cat{x: 10, age: 2});
    cats.push(Cat{x: 20, age: 3});
    cats.push(Cat{x: 30, age: 4});
    cats.push(Cat{x: 40, age: 5});
    cats.push(Cat{x: 50, age: 6});

    let which = 3;

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
