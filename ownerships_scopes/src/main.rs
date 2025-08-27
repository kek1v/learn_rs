fn main(){
    let s = String::from("Hello");

    takes_ownership(s); //s value moves in the function and not returned
                        //.. s's value is not valid here


    let x = 5; //x dont move into function (Copy trait)

    makes_copy(x);
    // x still be valid
}//x goes out of scope, then s

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}// here some string goes out of scope and drop fn has been called
 // memory is freed
 

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}//here some integer goes out of scope. Nothing special happens
