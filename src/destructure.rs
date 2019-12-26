#[allow(dead_code)]

fn main() {
    let pair = (2,3);

    // destructing tuples
    println!("Tell me about {:?}", pair);
    match pair {
        (0,y) => println!("First is `0` and `y` is `{:?}`", y),
        (x,0) => println!("`x` is `{:?}` and last is `0`",x),
        _ => println!("It does not matter what they are")
    }


    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32,u32,u32),
        HSV(u32,u32,u32),
        HSL(u32,u32,u32),
        CMY(u32,u32,u32),
        CMYK(u32,u32,u32,u32)
    }

    let color = Color::CMY(122,17,40);
    // enum destructing
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r,g,b) => println!("Red : {} Green : {} Blue : {}",r,g,b),
        Color::HSV(h,s,v) => println!("Hue : {} Saturation : {} Value : {}", h,s,v),
        Color::HSL(h,s,l) => println!("Hue : {} Saturation : {} Lightness : {}", h,s,l),
        Color::CMY(c,m,y) => println!("Cyan : {} Magenta : {} Yellow : {}", c,m,y),
        Color::CMYK(c,m,y,k) => println!("Cyan : {} Magenta : {} Yellow : {} Key : {}", c,m,y,k)
    }
    // all Color variables are covered. no need for _

    //pointer destructuring

    // & makes reference being assigned
    let reference = &4i32;

    match reference {
        // &val holds reference pattern 
        &val => println!("Got a value via destructuring : {:?}", val)
    }

    // * ensures dereference and match pattern. so & is not required
    match *reference {
        val => println!("Got a value via dereferencing : {:?}",val)
    }

    let _not_a_reference = 3;

    //creating reference without &
    let ref _is_a_reference = 3;

    match _is_a_reference {
        &val => println!("Got a reference to value : {}", val)
    }

    // same semantic as above line
    match _is_a_reference {
        ref r => println!("Got a reference to value : {}", r)
    }

    let mut mut_value = 6;

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}",m)
        }
    }


    //destructuring struct

    struct Foo {
        x: (u32,u32),
        y: u32,
    }

    let foo = Foo { x: (1,2), y: 10};

    match foo {
        Foo {x: (1,b), y} => println!("First of x is 1 b = {:?}, y = {:?}",b,y),

        // order does not matter
        Foo {y: 3, x: i} => println!("y is 3, i = {:?}",i),

        // ignore other variables except y
        Foo { y, ..} => println!("y = {}, we don't care about x", y),

        //missing field produces error
        //Foo { y } => println!("y = {}",y)
    }

}