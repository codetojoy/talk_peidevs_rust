
mod eg_animal_lifetime;

use eg_animal_lifetime::Cat;
use eg_animal_lifetime::Dog;
use eg_animal_lifetime::oldest;

fn run_eg_animal_lifetime() {
    let brahms = Cat {name: String::from("Brahms"), age: 3, .. Cat::new() };
    let chopin = Dog {name: String::from("Chopin"), age: 4, .. Dog::new() };
    let result = oldest(&brahms, &chopin);
    println!("TRACER result: {}", result.to_string());
}

fn main() {
    run_eg_animal_lifetime();
    println!("TRACER Ready.");
}
