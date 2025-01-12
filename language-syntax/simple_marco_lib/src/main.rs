
use simple_marco_lib::niels::{test_lib, without_mod_fn};

fn main() { 
    test_lib::public_function();
    test_lib::indirect_access();
    without_mod_fn();

    println!("Hello, world!");

}
