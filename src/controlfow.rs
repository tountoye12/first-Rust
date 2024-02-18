


pub fn print_array(mut arr: &[u32]) {

    //let arr = [10, 20, 30, 40, 50];
    // for i in arr {
    //     println!("{} ", i);
    // }
    for element  in arr.iter(){ 
        println!("The value is {} ", element)
    }


}

// pub fn copy_array_to_vec(mut arr: &[u32]) {

//     let mut vec:Vec<&u32> = Vec::new();

//     for value  in arr {
//         vec.push(value);
//     }

// }