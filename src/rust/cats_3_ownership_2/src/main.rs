
#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn do_log(cat: Cat) {
    println!("TRACER {:?}", cat);
}

fn main() {
    let mut cat1 = Cat {name: String::from("Mozart"), age: 4};
    cat1.age += 1;
    do_log(cat1);

    // !!!
    // println!("TRACER {:?}", cat1);
}






