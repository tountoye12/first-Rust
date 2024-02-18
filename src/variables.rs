
pub fn run() {

    // Fix the error below with least amount of modification to the code
    // let x: i32 = 5; // Uninitialized but used, ERROR !
    // let y: i32;// Uninitialized but also unused, only a Warning !

    // assert_eq!(x, 5);
    // println!("Success");

    // Fill the blanks in the code to make it compile
    // let  mut i: i32 = 1;
    // i += 2;
    // assert_eq!(i, 3);
    // println!("Success");

    // Fix the error below with least amount of modification
    let x: i32 = 10;

    {
        let y: i32 = 5;

        println!("The value of x is {} and the value of y is {}", x, y);
    }
    // println!("The value of x is {} and the value of y is {}", x, y);



}

pub fn print_name() {
    println!("Diallo");
}