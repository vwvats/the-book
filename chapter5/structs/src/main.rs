// Structs are like Tuples insofar that they group related data
// and data can be of different types
// But in Structs the data are named

// use the Struct keyword to define a new struct
// define key names and annotate their types
#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u32,
    is_logged_in: bool,
}

fn main() {
    // to create an instance of a struct just assign it to a variable
    // and fill in the key fields
    let user0 = User {
        first_name: String::from("Vivek"),
        last_name: String::from("Vats"),
        age: 26,
        is_logged_in: true,
    };

    // special formatting is required to print custom types :? or :#?
    println!("{:?}", user0);

    // value of a key can be changed, however the entire struct needs to be mutable
    let mut user1 = User {
        // an instance of struct can be copied into another with ..
        ..user0 // btw this moves the ownership
        // it won't override what's explicity defined though
        is_logged_in: false,
    };
    user1.first_name = String::from("Vivec");
    println!("{:?}", user1);

    // functions can be used as struct builders
    fn build_user(first_name: String, last_name: String) -> User {
        User {
            // when parameter and key names match then passing the parameter as value
            // can be omitted i.e. first_name: first_name
            first_name,
            last_name,
            age: 42,
            is_logged_in: false,
        }
    }

    // Tuple Struct - way to define Tuples with similar data
    struct Color(u32, u32, u32);
    struct Size(u32, u32, u32);
    // for ordinary Tuples these would mean the same thing
    // but for Struct Tuples, Rust is going to enforce types
}
