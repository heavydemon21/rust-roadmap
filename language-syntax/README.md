# Learning rust basics
In this chapter i will write down all the things i learned from the book

## [getting started](https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html)

### Compiling
this will compile a *Hello World* example
```bash
rustc main.rs
```
Where you then can run and output **Hello World**

```bash
./main
```

### Cargo
Instead of manually running code can cargo be used. [Cargo](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) is the package manager for rust. These are the commands.

```bash
// creates a new directory(hello_cargo) with Cargo.toml and a src/main.rs
$ cargo new hello_cargo

// compile 
$ cargo check 

// building
$ cargo build

// run
$ ./target/debug/hello_cargo

// build and run
$ cargo run

// released cargo run
$ cargo run --release
```





