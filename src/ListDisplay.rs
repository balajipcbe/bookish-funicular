use std::fmt;

//define a structure containing vector 
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the vector ref
        let vec = &self.0;

        write!(f,"[")?;

        for (count, v) in vec.iter().enumerate() {
            //add commma all items except first item
            if count != 0 { write!(f, ", ")?;}
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}