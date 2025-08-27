fn main() {
    let s1 = String::from("Hello ");

    //let mut s2 = s1; its a move

    let mut s2 = s1.clone();


    s2.push_str(", world!");
    println!("{s2}");
}
