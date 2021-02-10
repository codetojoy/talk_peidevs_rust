
mod eg_animal_lifetime;

use eg_animal_lifetime::Cat;
use eg_animal_lifetime::Dog;
use eg_animal_lifetime::oldest;

fn run_lifetime() {
    let mozart = Cat {name: String::from("Mozart"), age: 3};
    let beethoven = Dog {name: String::from("Beethoven"), age: 4};
    let result = oldest(&mozart, &beethoven);
    println!("TRACER result: {}", result.name());
}

fn main() {
    run_lifetime();
    println!("TRACER Ready.");
}








/*
fn eg_2() {
    let mozart = Cat {name: String::from("Mozart"), age: 3, .. Cat::new() };
    let result;
    {
        let beethoven = Dog {name: String::from("Beethoven"), age: 4, .. Dog::new() };
        result = oldest(&mozart, &beethoven);
    }
    println!("TRACER result: {}", result.to_string());
}
*/
