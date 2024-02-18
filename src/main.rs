
#![allow(warnings)]
mod variables;
mod literals;
mod tuples;
mod arrays;

mod controlfow;

fn main() {
   
    // variables::run();
    // variables::print_name();

    // literals::literal();
   // tuples::tuple();
    let slice:[i32; 7] = [1, 3, 4, 10, 4, 5, 20];
    let arr = [10, 20, 30, 40, 50];
    // arrays::analyze_slice(&slice);

    controlfow::print_array(&arr);



    

}
