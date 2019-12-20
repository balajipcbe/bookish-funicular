fn f1() {
    let names = vec!["Bob","Frank","Ferris"];
    for name in names.iter() {
        //immutable reference
        match name {
            &"Ferris" => println!("there is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }

    // names still exists
    println!("{:?}", names);
}


fn f2() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}",name),
        }
    }
    //uncommenting following line produces error. bcos vec is moved.
    //println!("{:?}",names);
}


fn f3() {
    let mut names = vec!["Bob","Frank","Ferris"];

    for name in names.iter_mut() {
        //mutable reference
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us",
            _ => "Hello",
        }
    }
    println!("names : {:?}", names);
}

fn main() {
    f1();
    f2();
    f3();

}