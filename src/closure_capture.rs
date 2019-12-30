fn main() {
    use std::mem;

    let color = "green";

    // immutable reference
    let print = || println!("color : {}", color);

    let _reborrow = &color;

    let _color_moved = color;

    print();

    println!("_moved_color : {}", _color_moved);


    // mutable reference
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count : {}", count);
    };

    inc();

    //let _reborrow = &count;

    inc();

    let _count_reborrowed = &mut count;


    //move 

    let movable = Box::new(3);

    let consume = || {
        println!("movable : {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();

    let haystack = vec![1,2,3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    //uncommenting below line produces compile error
    //bcos it uses after move in to closure
    //println!("{}", haystack.len());
}