use std::io;

fn check_interval(number: i32, start: i32, end: i32) -> bool{
    if number >= start && number <= end {
        return true;
    }
    return false;
}

fn main() {
    println!("==> Level up !");
    println!("Number to verify : ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read number");
    let number : i32 = match number.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("failed to get number from input !")
    };
    loop{
        println!("Input the interval start number : ");
        let mut start: String = String::new();
        io::stdin().read_line(&mut start).expect("Failed to read start number for interval");
        let start: i32 = match start.trim().parse::<i32>(){
            Ok(num)=> num,
            Err(_) => panic!("Failed to parse start interval input to number !"),
        };
        println!("Input the interval start number : ");
        let mut end:String = String::new();
        io::stdin().read_line(&mut end).expect("Failed to read end for interval !");
        let end: i32 = match end.trim().parse::<i32>(){
            Ok(num) => num,
            Err(_) => panic!("Failed to parse end interval input !")
        };
        if start > end {
            println!("\nThe start must be less than the end of the interval");
            continue;
        }
        
        if check_interval(number, start, end){
            println!("{} is in the interval [{} ; {}]", number, start, end);
        }
        else{
            println!("{} is not in the interval [{} ; {}]", number, start, end);
        }
        break;
    }
   
    
}
