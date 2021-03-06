fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of rect {:#?} is {}", rect, area(&rect))
}
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}