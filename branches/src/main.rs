fn main() {

    /*
    let x: u128 = 52;
    
    if x % 4 == 0{
        println!("number is divisible by 4!");
    } else if x % 3 == 0{
        println!("number is divisible by 3!");
    } else if x % 2 == 0{
        println!("number is divisible by 2!");
    } else{
        println!("number is not divisible by 4, 3 or 2!");
    }
    */

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
}
