
/*
mod eg_lifetime;

fn run_eg_lifetime() {
    let p = eg_lifetime::Person {name: String::from("Mozart"), birth_year: 1956, .. eg_lifetime::Person::new() };
    let s = String::from("5150");
    let t = String::from("Van Halen");
    let result = eg_lifetime::longest_with_an_announcement(&s, &t, p);
    println!("TRACER result: {}", result);
}
*/

mod eg_animal_lifetime;

fn run_eg_animal_lifetime() {
    let brahms = eg_animal_lifetime::Cat {name: String::from("Brahms"), age: 3, .. eg_animal_lifetime::Cat::new() };
    let chopin = eg_animal_lifetime::Cat {name: String::from("Chopin"), age: 4, .. eg_animal_lifetime::Cat::new() };
    let result = eg_animal_lifetime::oldest(&brahms, &chopin);
    println!("TRACER result: {}", result);
}

fn main() {
    run_eg_animal_lifetime();
    println!("TRACER Ready.");
}
