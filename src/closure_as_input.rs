fn apply<F>(f: F) where
    F: FnOnce() {
        f();
    }


fn apply_to_3<F>(f: F) -> i32 where 
    F: FnOnce(i32) -> i32 {
        f(3)
    }

fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let dairy = || {
        // greeting is by reference
        println!("I said {}.", greeting);

        //farewell mutable reference
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");

        //calling drop forces farewell by value
        //it needs to be called by FnOnce
        mem::drop(farewell);
    };
    
    apply(dairy);

    let double = |x| 2 * x;

    println!("3 doubled : {}", apply_to_3(double));
}