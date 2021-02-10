
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn main() {
    // borrowing: mutable reference
    let mut cat1 = Cat {name: String::from("Mozart"), age: 4};
    let cat2 = &mut cat1;
    // can't have immutable reference and mutable reference
    // !!!
    // let cat3 = &cat1;

    cat2.age += 1;
    println!("TRACER {:?}", cat2);
}
