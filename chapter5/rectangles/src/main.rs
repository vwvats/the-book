// An example program using structs add readability to code

struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation of a Struct contains its methods
// methods can have same names as struct fields
// however, same names ideally should be used as getters
impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

// An associated function used as a constructor
// calling Rectangle::square(10) will return an instance of Rectangle 
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}