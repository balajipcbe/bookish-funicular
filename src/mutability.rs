fn main() {
    let _immuatable_var = 1;
    let mut _mutable_var = 1;

    println!("mutable var before mutation : {}", _mutable_var);
    _mutable_var += 1;
    println!("mutable var after mutation : {}", _mutable_var);

    //error
    //_immuatable_var = 1;

    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;

        println!("Inner scope short_lived_binding : {:?}", short_lived_binding);

        println!("Inner scope long_lived_binding : {:?}", long_lived_binding);

        let long_lived_binding = 2;

        println!("Inner scope long_lived_binding after shadowing : {:?}", long_lived_binding);
    }

    //error
    //println!("Outer scope short_lived_binding : {:?}", short_lived_binding);

    println!("Outer scope long_lived_binding : {:?}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("Outer scope long_lived_binding after shadowing : {:?}", long_lived_binding);

    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a_binding : {}", a_binding);

    let another_binding;

    //println!("another_binding : {}", another_binding);

    another_binding = 1;

    println!("another_binding after init : {}", another_binding);

}