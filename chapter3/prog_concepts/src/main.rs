fn main() {

    // 1. Variables
        // immutable by default
        // wrong code below - compiler will throw error
        let num = 69;
        println!("{}", num);
        num = 42;
        println!("{}", num);

        // prepend mut keyword to make variables mutable
        // right code below
        let mut mutable_num = 69;
        println!("{}", mutable_num);
        mutable_num = 42;
        println!("{}", mutable_num);

        // constants can never be mutable and have to be type annotated (no inference)
        // convention is to use UPPER_SNAKE to declate constants
        const CONST_NUM: u32 = 148_000;

        // variable can be shadowed using let keyword again
        let num = 6;
        println!("{}", num);
        let num = "six";
        println!("{}", num);
        // both 6 and six will be printed
    
    // 2. Data Types
        // Scalar
            // I) Integer
                // numbers without a fractional part
                // inferred as i32 by default if unannotated
                // can be signed or unsigned
                // can be 8 (i8, u8), 16 (i16, u16), 32, 64, or 128 bits
                // can also take size from architecture (isize, usize)
                // can be written in decimal, hex, octal, binary, and byte forms
                // can overflow (256 == 1 for u8) if the type is too small for the number
            // II) Floating Point Number
                // numbers with a fractional part
                // inferred as f64 by default if unannotated
                // can be f32 or f64 
                // standard mathematical operations - + , - , * , /, %
            // III) Boolean
                // represents true or false
            // IV) Character
                // represents a unicode character and written with a single quote ''
        // Compound 
            // I) Tuple
                // fixed sized collection of related data
                // data can be of different types
                let person_tuple = ("First Last", 26, "Company Inc");
                // can be destructured into variables
                let (name, age, company) = person_tuple;
                // can be accessed with dot notation on index
                let name = person_tuple.0;
                // an empty tuple has one value () and is called the unit type
            // II) Array
                // fixed sized collection of related data
                // data has to be of the same type
                // can be declared literally 
                let some_array = [1, 2, 3];
                // can also be declared using speical syntax
                let array_with_eight_zeroes = [0, 8];
                // the above code creates an array of length 8 and fills it with 0s
                // can be accessed with bracket notation on index
                let zero = array_with_eight_zeroes[4];

    // 3. Functions
        // declared with the fn keyword
        fn some_function() { }
        // parameters have to be annotated
        // can take in list of comma separated arguments
        fn another_func(arg1: i32, arg2: i32) { println!("arg1 {}, arg2 {}", arg1, arg2) }
        // requires a slim arrow to define a return type
        fn yet_another_func(arg1: i32, arg2: i32) -> i32 { arg1 + arg2 }
        // return keyword is not required above because expressions are implicitly returned
        // semi colon is omitted for expression because adding it makes it a statement
    
    // 4. Control Flow
        // Conditional 
            if 2 > 1 { println!("2 is greater than 1"); }
            // parentheses in condition not required 
            // the condition should be explicitly boolean
            // can be used inside of a variable declaration
            let uknown = if 3 == 4 { 42 } else { 69 };
        // Loops
            // will continue looping until broken out of
            loop { break; }
            // will continue looping as long as the cond is true
            while 2 > 3 { break; } 
            // loop over iterables using for in with .itr()
            for num in array_with_eight_zeroes.iter() {
                println!("{}", num);
            }
            // loop over a range
            for num in 1..4 { println!("{}", num); }
    
    // 5. Commenting
        // single line
        /* multiple
        line */
}
