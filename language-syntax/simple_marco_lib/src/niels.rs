
pub mod test_lib {

pub fn public_function() {
    println!("called niels `public_function()`");
}

fn private_function() {
    println!("called niels `private_function()`");
}

pub fn indirect_access() {
    print!("called niels `indirect_access()`, that\n> ");

    private_function();
}

}

pub fn without_mod_fn(){
    println!("called niels `without_mod_fn()`");
}
