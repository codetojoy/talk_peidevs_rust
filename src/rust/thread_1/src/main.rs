use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // `move` is crucial for this to work
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}