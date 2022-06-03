1. A package is a set of crates providing a certain functionality. Each package has a Cargo.toml file.

2. A crate can be a binary crate or a library crate:
    - binary crates can be compiled to an executable, they have a main function that runs when executed
    - library crates define functionality (code) that can be shared with other projects
    - crate root is the path serving as the entrypoint for the rust compiler
    - the binary crate root is src/main.rs and the library crate root is src/lib.rs
    - crate names as same as the package name

3. A package has to contain at least one crate, furthermore:
    - it cannot have more than 1 library crate
    - it can, however, have as many binary crates as needed
    - additional binary crates can be placed in the src/bin directory wherein each file is a crate 