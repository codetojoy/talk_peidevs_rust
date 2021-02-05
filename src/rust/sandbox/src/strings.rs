
pub fn run() {
    let mut s = String::from("abc");
    s.push_str("def");
    println!("TRACER strings");
    println!("TRACER s: {}", s);
}