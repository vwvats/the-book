// An enum type is defined by listing all of its variants
#[derive(Debug)]
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Windy,
}

fn main() {
    // enums can be assigned to a variable
    let current_weather = Weather::Windy;
    println!("{:?}", current_weather);

    // enums can be simple or complex
    #[derive(Debug)]
    enum IpAddrType {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // when assigning to a variable, value has to be defined
    // the variant here is a function that constructs an instance of the enum
    let home = IpAddrType::V4(127, 0, 0, 1);
    let office = IpAddrType::V6(String::from("whatever"));

    // enums can also have methods
    impl IpAddrType {
       fn scream(&self) {
           println!("{:?} screams!", self);
       } 
    }
    office.scream();

    // Option type
    // Rust has a special enum type called Option
    // it can take a value of any type (Some) or be null (None)
    let option_string = Some("whatever");       // type = Option<&str>
    let option_integer = Some(42);              // type = Option<i8>
    let option_null_string: Option<u32> = None; // type = Option<u32> notice type annotation is required here
    // this is to avoid null values
    // since we know beforehand that only Option types can be potenially null

    // Note that Option<&str> and &str are not the same
    // the former can be absent while the latter always has a valid value
    // in order to perform operations, Option<T> has to be converted to T first
}
