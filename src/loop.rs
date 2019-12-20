#![allow(unreachable_code)]

fn main() {
    let mut count = 0u32;

    println!("Lets count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}",count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }


    //labelling loop
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("this point will never be reached");
    }
    println!("Exited the outer loop");


    //return from loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    assert_eq!(result, 20);
    println!("Result is {}", result);

    //while loop example

    let mut n = 0u32;

    while n < 101 {
        if n % 15 == 0 {
            println!("Fizzbuzz");
        }else if n % 3 == 0 {
            println!("Fizz");
        }else if n % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}",n);
        }
        n += 1;
    }

    // for and range example
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("FIZZBUZZ");
        }else if n % 3 == 0 {
            println!("FIZZ");
        }else if n % 5 == 0 {
            println!("BUZZ");
        }else {
            println!("{}",n);
        }
    }
}