use std::io;

pub fn hello(name:String){

    println!("Hello my name is {}", name);
    
}


pub fn get_input() -> String{
    
    println!("type your name");
    let mut input  = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    //println!("My name is {}", input);
    input
}