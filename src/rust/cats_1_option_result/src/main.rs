
/*
Option<T>
    Some(T)
    None

Result<T,E>
    Ok(T)
    Err(E)
*/

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

fn find_cat_by_name(cats: Vec<Cat>, name: &str) -> Option<Cat> {
    let mut result = None;
    for cat in cats {
        if cat.name == name {
            result = Some(cat);
        }
    }
    return result;
}

fn fetch_cat(with_error: bool, name: &str) -> Result<Cat, String> {
    let result;

    if with_error {
        result = Err(String::from("fetch failed"));
    } else {
        result = Ok(Cat{name: String::from(name), age: 0});
    }

    return result;
}

fn handle_option(opt: Option<Cat>) {
    match opt {
        Some(cat) => println!("TRACER {:?}", cat),
        None => println!("TRACER no cat")
    }
}

fn handle_result(result: Result<Cat,String>) {
    match result {
        Ok(cat) => println!("TRACER {:?}", cat),
        Err(why) => println!("TRACER error: {}", why)
    }
}

fn build_cats() -> Vec<Cat> {
    let mut cats = vec![];
    cats.push(Cat{name: String::from("Bach"), age: 1});
    cats.push(Cat{name: String::from("Beethoven"), age: 2});
    cats.push(Cat{name: String::from("Mozart"), age: 3});
    cats.push(Cat{name: String::from("Vivaldi"), age: 2});
    cats.push(Cat{name: String::from("Brahms"), age: 3});
    cats.push(Cat{name: String::from("Chopin"), age: 4});
    cats.push(Cat{name: String::from("Stravinsky"), age: 1});
    cats.push(Cat{name: String::from("Shostakovich"), age: 2});
    return cats;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let which = args[1].parse::<u8>().unwrap();
    println!("TRACER running case: {}", which);

    if which == 1 {
        // Option: happy path
        let cats = build_cats();
        let opt = find_cat_by_name(cats, "Chopin");
        handle_option(opt);
    } else if which == 2 {
        // Option: error
        let cats = build_cats();
        let opt = find_cat_by_name(cats, "BOGUS");
        handle_option(opt);
    } else if which == 3 {
        // Option: panic
        let cats = build_cats();
        let opt = find_cat_by_name(cats, "BOGUS");
        println!("TRACER: {:?}", opt.unwrap());
    } else if which == 4 {
        // Result: Ok
        let with_error = false;
        let result = fetch_cat(with_error, "Paganini");
        handle_result(result);
    } else if which == 5 {
        // Result: None
        let with_error = true;
        let result = fetch_cat(with_error, "BOGUS");
        handle_result(result);
    } else if which == 6 {
        // Result: expect happy path
        let with_error = false;
        let result = fetch_cat(with_error, "Paganini");
        println!("TRACER: {:?}", result.expect("hello if error"));
    } else if which == 7 {
        // Result: expect panic
        let with_error = true;
        let result = fetch_cat(with_error, "BOGUS");
        println!("TRACER: {:?}", result.expect("hello if error"));
    }

    println!("Ready.");
}
