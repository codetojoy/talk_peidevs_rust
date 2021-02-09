
impl Cat {
    pub fn new() -> Cat {
        Cat {
            name: String::from(""),
            age: 0,
        }
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
    // let cat2 = Cat {name: String::from("Chopin"), age: 3, .. Cat::new()};
    cats.push(cat1);
    let mozart = &cats[0];
    // !!!
    // cats.push(cat2);
    println!("TRACER {:?}", mozart);
}
