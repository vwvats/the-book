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
}
