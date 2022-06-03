1. A Package is a set of crates providing a certain functionality. Each package has a Cargo.toml file.

2. A Crate can be a binary crate or a library crate:
    - binary crates can be compiled to an executable, they have a main function that runs when executed
    - library crates define functionality (code) that can be shared with other projects
    - crate root is the path serving as the entrypoint for the rust compiler
    - the binary crate root is src/main.rs and the library crate root is src/lib.rs
    - crate names as same as the package name

3. A Package has to contain at least one crate, furthermore:
    - it cannot have more than 1 library crate
    - it can, however, have as many binary crates as needed
    - additional binary crates can be placed in the src/bin directory wherein each file is a crate 

4. Modules are used to organize code in crates. The Rust compiler looks for modules in the following order:
    - directly following the module definition
    ```rust mod house { } ```
    - if the module definition ends with a semi colon then looks for src/house.rs 
    ```rust mod house; ``` 
    - if no such file exists, then it looks for src/house/mod.rs
    - modules can have sub-modules, the Rust compiler looks for sub-modules in the following order:
        - directly following the sub-module definiton (if no semi colon), then
        - src/house/room.rs
        - src/housr/room/mod.rs

5. Modules are private fromt its parent by default, use the ```pub``` keyword to make them public.

6. Use the ```use``` keyword to create shortcuts and reduce repition of long paths 
    ```rust 
        crate::house::room::some_func_in_house();
        // can be reduced to
        use crate::house::room::some_func_in_house;
        pub house;
        some_func_in_house();
    ```