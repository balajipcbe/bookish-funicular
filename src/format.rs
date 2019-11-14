

fn format_examples() 
{
    // here 31 integer is automatically stringified
    println!("{} days", 31);

    //positional arguments
    println!("{0}, This is {1}. {1}, This is {0}","Alice","Bob");

    //named arguments
    println!("{subject} {verb} {object}",verb="love",subject="I",object="you");

    //special formatting happens after colon :
    println!("{} of {:b} people know binary, other half doesn't",1,2);

    //right alignment text : 5 spaces and 1 
    println!("{number:>width$}",number=1,width=6);

    //prepadding with zeros
    println!("{number:>0width$}",number=1,width=6);

    //formatting floating point to 3 digit after decimal
    println!("{}",format!("{:.*}",3,3.143267));

    //formatting user-defined types with debug attribute
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("Now {:?} will print!", Structure(7));

    println!("Now {:?} will print!", Deep(Structure(99)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age:u8
    }

    //pretty print
    let name = "Peter";
    let age = 25;
    let peter = Person { name, age};
    println!("{:#?}",peter);
}

fn main()
{
    format_examples();
}