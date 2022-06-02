// An example program using structs add readability to code

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle0 = Rectangle {
        width: 100,
        height: 75,
    };
    
    let area0 = calculate_area(&rectangle0);
    println!("The area of rectangle 0 is {}", area0);
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}