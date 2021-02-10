
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn main() {
    // borrowing: multiple immutable references
    let cat1 = Cat {name: String::from("Mozart"), age: 4};
    let cat2 = &cat1;
    let cat3 = &cat1;

    // can't move ownership when borrowed
    // !!!
    // let cat4 = cat1;
    // println!("TRACER {:?}", cat4);

    println!("TRACER {:?}", cat2);
    println!("TRACER {:?}", cat3);
}
