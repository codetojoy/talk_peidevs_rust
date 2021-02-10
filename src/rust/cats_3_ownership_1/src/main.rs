
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Cat {
    pub fn noise() -> String {
        return String::from("new");
    }
}

fn main() {
    let cat1 = Cat {name: String::from("Mozart"), age: 4};
    let cat2 = cat1;

    // !!!
    // println!("TRACER {:?}", cat1);
    println!("TRACER {:?}", cat2);
}











