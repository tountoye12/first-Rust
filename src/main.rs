
#![allow(warnings)]

use std::ffi::IntoStringError;
mod variables;
mod literals;
mod tuples;
mod arrays;

mod controlfow;
mod structs;

mod bah;

fn main() {


    let name = bah::get_input();

    bah::hello(name);


   
    // variables::run();
    // variables::print_name();

    // literals::literal();
   // tuples::tuple();
    let slice:[i32; 7] = [1, 3, 4, 10, 4, 5, 20];
    let arr: [u32; 5] = [10, 20, 30, 40, 50];
    // arrays::analyze_slice(&slice);

    //controlfow::print_array(&arr);


    let user1 = User{
        username : String::from("diallo"),
        email: String::from("diallo@gmail.com"),
        sign_in_id: 36,
        active: true
    };


}

struct User {
    username: String,
    email: String,
    sign_in_id: u64,
    active: bool
}
fn build_user(username:String, email:String, sign_in_id:u64, active:bool) -> User {

    User {
        email,
        username,
        sign_in_id,
        active
    }
}



