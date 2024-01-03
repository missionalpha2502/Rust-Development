fn main() {

    let mut x = 0;

    loop {

        x += 1;

        if x == 20 {
            println!("The loop will stop here");
            break;
        }
        if x % 3 !=0 {
            continue;
        }

        println!("{} is divisible by 3", x);
    }



}
