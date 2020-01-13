#![allow(dead_code)]



fn used_function() {}


// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}
