fn main() {
    println!("If/else Statement");

    let n = 3;

    if n< 10{
        println!("{} is less than 10", n);
    }


    println!("Loop Statements");

    let mut x = 0;

    loop{
        x += 1;

        if x>= 30{
            println!("x is now greater than or equal to 30 so we break the loop");
            break;
        }

        if x % 3 != 0 {
            continue; // the print statement following this if statement will not printed//
        }

        println!("{} is divisible by 3", x);
    }
}