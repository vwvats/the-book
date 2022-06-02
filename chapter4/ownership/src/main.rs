/*
Data that doesn't have a fixed known size has to be allocated on the heap.

There are two primary ways in which this has been handled so far:
    i) manual memory allocation & deallocation by the programmer
        C, C++
    ii) automatic allocation & deallocation with garbage collector
        Java, C#

Both approaches have their pros and cons:
    i) manual is error prone and slow to write 
        but runtime is fast/consistent and program size is smaller
    ii) automatic is error-free and fast to write 
        but runtime is slow/inconsistent and program size is larger

This is where Rust comes in with it's ownership model.
It gives the best of both worlds:
- faster runtime
- smaller program size
- error-free
at the expense of slow write time during development   
*/

fn main() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
        // therefore for the value to be used by someone who isn't its owner
        // ownership has to either be transferred or lent
    // 3. When the owner goes out of scope, the value will be dropped.
        // and the allocation will be freed
    {
    // PI has not been declared yet
        const PI = 22 / 7; // PI can be accessed from here to end of scope
    }
    // PI has been dropped and cannot be accessed here


    let simple = 42;
    let simple_copy = simple; 
    // 42 is copied directly as it's a simple value with a known fixed size


    let complex = String::from("Hello World!");
    let complex_transfer = complex;
    // "Hello World" is not deep copied as it's too expensive
    // "Hello World" is not shallow copied either due to the double free problem
        // when two pointers point to the same memory in heap
        // then automatic drop by Rust will try to free the same memory twice
    // What actually happens is that "Hello World" ownership is moved to complex_transfer
        // complex is not the owner now, complex_transfer is
    // To do an actual use, use the clone method 
    let complex = String::from("Hello Clone!");
    let complex_clone = complex.clone();


    // It is also possible to use a value without taking ownership of it
    // and this is done with references
    let some_string = String::from("hello");
    let length = calc_length(&some_string);
    fn calc_length(s: &String) -> usize {
        s.length()
    }
    // Here some_string is passed into the function calc_length
    // but the ownership is not transferred to the function


    // Note that references cannot be modified
    // fn try_modifying(&some_string) {
    //     some_string.push_str(" world!");
    // }
    // The code above will throw a compile error
    // References however can be made mutable with mut keyword
    fn try_modifying(&mut some_string) {
        some_string.push_str(" world!");
    }
    // Mutable references can be only used once in a scope to prevent race conditions
    // Mutable references cannot be combined with immutable refrences for the same reason
    

    // String Slices are built in methods in Rust because performing operations on strings 
    // manually is tedious and error prone
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // first, last, or both numbers can be dropped if:
        // string slice starts at index 0  -- [..4]
        // string slice ends at last index -- [3..]
        // string slice starts at 0 AND ends at last index -- [..]
    // string literals (&str) are just slices that's why they're immutable,
    // because they are a reference ponting to a slice in the binary
}   