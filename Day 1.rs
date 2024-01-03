fn main(){
    let x = 1;
    let y = 3; //i32
    let mut z = 10; //mut i32

    println!("Before reassignment: x is{}, y is {}, and z is {}", x, y, z);

    z = 12; //this is valid

    //x =7; This is invalid since x is not mutable,
    // so this won't compile

    let mut greeting = "World";

    println!("Hello {}", greeting);

    greeting = "Zephyr";

    println!("Hello {}", greeting);


}
