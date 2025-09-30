#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let output = rect1.area();
    println!("The area of the rectangle is {}", output);

    if rect1.width(){
        println!("The area of the rectangle is {}", rect1.width);
    }
}
