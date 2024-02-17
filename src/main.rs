
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age:u8
}

fn main() {
    // println!("Hello, world!");
    // println!("I'm Diallo and want to learn Rust");

    // let x = 5 + 90 + 5;

    // println!("Is `x` 10 or 100 ? x = {}", x);

    let name: &str = "diallo";
    let age = 33;
    let person = Person  {
        name,
        age
    };

    println!("{:#?}", person);

}
