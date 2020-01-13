fn destroy_box(c: Box<u32>) {
    println!("Destroying a box that contains {}", c);
}


fn main() {
    // x on stack
    let _x = 5u32;

    // copied data
    let _y = _x;

    println!("x: {} y: {}", _x,_y);

    let a = Box::new(4u32);

    println!("a contains : {}", a);

    // move, copied the pointer address of a to b 
    let b = a;
    // Error , a does not own this address 
    //println!("a contains : {}", a);
    println!("b contains : {}", b);

    destroy_box(b);

    //Error, bcos using after free
    //println!("b contains : {}", b);
}