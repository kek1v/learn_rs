fn main() {
    let s = String::from("hello");
    let l = check_lenght(&s);
    println!("{l}");

}

fn check_lenght(some_str: &String) -> usize {
    some_str.len()
}
