#![allow(overflowing_literals)]

//aliasing example
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;


fn main() {
    let decimal = 65.4321_f32;

    //casting
    let _integer:u8 = decimal as u8;

    let _character: char = _integer as char;
    println!("casting {}->{}->{}", decimal,_integer,_character);

    println!("1000 fits in u16 {}", 1000 as u16);

    //drops 8 MSB bits in u16 that yields 232
    println!("1000 as u8 is {}", 1000 as u8);

    println!("-1 as u8 is {}", -1i16 as u8);

    println!("1000 % 256 is {}", 1000 % 256);

    println!("128 as i16 is {}", 128 as i16);

    //does 2's complement of 128
    println!("128 as i8 is {}", 128 as i8);

    println!("1000 as u8  is {}", 1000 as u8);

    println!("232 as i8 is {}", 232 as i8);


    //inference example

    let elem = 16u8;

    let mut vec = Vec::new();

    //commenting the below line stops defining the type of vector.
    //so oops error
    vec.push(elem);

    println!("{:?}", vec);


    //aliasing example again

    let nanoseconds : NanoSecond = 5 as u64_t;
    let inch : Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} units", nanoseconds, inch, nanoseconds+inch);


}