fn main() {
    another_fn(x_return(), y_return());
}

fn another_fn(x: u128,y: char){
    println!("another function x value is {x}{y}");
}

fn x_return() -> u128  {
    return 52;
}

fn y_return() -> char {
    return 'h';
}
