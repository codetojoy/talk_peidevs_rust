
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

fn do_log(cat: Cat) {
    println!("TRACER {:?}", cat);
}

fn main() {
    let cat1 = Cat {name: String::from("Mozart"), age: 4, .. Cat::new()};
    do_log(cat1);

    // !!!
    // println!("TRACER {:?}", cat1);
}






