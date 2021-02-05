
pub fn run() {
    let mut count = 1;

    while count <= 46 {
        let mut s = String::from("");

        if count % 3 == 0 {
            s.push_str("fizz");
        }

        if count % 5 == 0 {
            s.push_str("buzz");
        }

        if !s.is_empty() {
            println!("{} {}", count, s);
        }

        count += 1;
    }
}