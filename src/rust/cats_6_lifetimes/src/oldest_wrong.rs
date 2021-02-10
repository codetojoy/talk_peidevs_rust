

/*
*/
pub fn oldest(x: &impl Animal, y: &impl Animal) -> &dyn Animal
{
    println!("x: {}", x.noise());
    println!("y: {}", y.noise());
    if x.age() > y.age() {
        x
    } else {
        y
    }
}