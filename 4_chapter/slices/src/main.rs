fn main() {
    /*
    let s = "Hello world";
    let hello = first_word(&s);
    s.clear(); // now is invalid because s is empty but hello = 5
    */

    let s = "hello world";
    let hello = &s[..5]; //slice
}

/* without str slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}*/

