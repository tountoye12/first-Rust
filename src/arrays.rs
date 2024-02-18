

use std::mem;

// This function borrows a slice.
pub fn analyze_slice(slice:&[i32]){

    println!("First element of slice is {}", slice[0]);
    print!("Slice len is {}", slice.len());
}