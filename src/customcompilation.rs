#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}



// compile with --cfg flag 
// rustc --cfg some_condition .\customcompilation.rs
fn main() {
    conditional_function();
}