fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => println!("This is a really long string and {:?}",i),
        _ => ()
    } 

    //cleaner approach

    let number = Some(7);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    //does not match
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // ok failure happened. checking alternate case
    if let Some(i) = letter {
        println!("Matched {:?}",i);
    }else {
        println!("Did not match a number. Let's go with a letter");
    }

    let i_like_letters = false;

    //after 2 consecutive failure, goes to default
    if let Some(i) = emoticon {
        println!("Matched i {:?}", i);
    }else if i_like_letters {
        println!("Dint match with a number. Lets go with letter");
    }else {
        println!("Dint match with a letter. Lets go with emoticon");
    }

    //iflet <-> enum has a special behavior
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(3);

    // enum allows if let only to compare.
    // becuase `if` needs implemenation of PartialEq for Foo.
    if  let Foo::Bar = a {
        println!("a is FooBar");
    }

    if let Foo::Bar = b {
        println!("b is FooBar");
    }

    if let Foo::Qux(val @ 3) = c {
        println!("c is FooQux of {:?}", val);
    }
}