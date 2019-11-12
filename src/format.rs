

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
}

fn main()
{
    format_examples();
}