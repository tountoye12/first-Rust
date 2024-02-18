

pub fn copy_array_to_element_vec (mut my_array:&[u32]) -> Vec<&u32>{
    let mut vec: Vec<&u32> = Vec::new();
    
    for value in my_array.iter() {
        vec.push(value);
    }
    vec
}