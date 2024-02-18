
#![allow(warnings)]


use std::ffi::IntoStringError;
mod variables;
mod literals;
mod tuples;
mod arrays;

mod controlfow;
mod structs;

mod bah;

mod collections;

fn main() {

    //let name = bah::get_input();
    //bah::hello(name);
    // variables::run();
    // variables::print_name();

    // literals::literal();
   // tuples::tuple();
    let slice:[i32; 7] = [1, 3, 4, 10, 4, 5, 20];
    let arr: [u32; 5] = [10, 20, 30, 40, 50];
    // arrays::analyze_slice(&slice);

    let vec = collections::copy_array_to_element_vec(&arr);
    
    println!("Vec is : {:#?}", vec);

    let user1 = User{
        username : String::from("diallo"),
        email: String::from("diallo@gmail.com"),
        sign_in_id: 36,
        active: true
    };

    let user2 = build_user(String::from("Diallo"), String::from("diallo@gmail.com"));

    //println!("User 2 User name is : {:#?}", user2.username);

    user2.print_user();


}


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_id: u64,
    active: bool
}
fn build_user(username:String, email:String, ) -> User {
    User {
        email: email,
        username: username,
        sign_in_id: 1,
        active: true
    }
}

impl  User {
    fn print_user(&self) {
        println!("User is : {:#?}", self)
    }
    
}



