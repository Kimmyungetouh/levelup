use std::io;

fn check_prod(first_number:i32, last_number:i32) -> bool{
    if first_number < 0 && last_number < 0 || first_number > 0 && last_number > 0{
        return true ;
    }
    return false;
}

fn main() {
    println!("Level up !");
    let mut first_number: String = String::new();
    println!(" Enter the first number : ");
    io::stdin().read_line(&mut first_number).expect("Failed to read first number");
    let first_number: i32 = match first_number.trim().parse::<i32>(){
        Ok(num) => num,
        Err(_) => panic!("Failed to parse first_number to integer")
    };

    let mut last_number: String = String::new();
    println!("Enter the second number : ");
    io::stdin().read_line(&mut last_number).expect("Failed to read the last number !");
    let last_number : i32 = match last_number.trim().parse::<i32>(){
        Ok(num) => num,
        Err(_) => panic!("Failed to parse last_number to integer")
    };

    if check_prod(first_number, last_number){
        println!("The product of {} and {} is positive !", first_number, last_number);
    }else {
        println!("the product of {} and {} is negative !", first_number, last_number);
    }
}


