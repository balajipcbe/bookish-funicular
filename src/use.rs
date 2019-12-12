#![allow(dead_code)]

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Solider
}

//implicit discriminator
enum Number {
    Zero,
    One,
    Two
}

//explicit discriminator
enum Color {
    RED = 0xff0000,
    GREEN = 0x00ff00,
    BLUE = 0x0000ff,
}

fn main() {
    use crate::Status::{Rich, Poor};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Status::Poor => println!("Poor people do not have money"),
        Status::Rich => println!("Rich people have lot of money"),
    }

    match work {
        Work::Civilian => println!("Civilians work"),
        Work::Solider => println!("Soliders fight")
    }

    //enums casted to i32
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    println!("Two is {}", Number::Two as i32);

    println!("Rose is 0x{:06x} color", Color::RED as i32);
    println!("Leaf is 0x{:06x} color", Color::GREEN as i32);
    println!("Sky is 0x{:06x} color", Color::BLUE as i32);
}