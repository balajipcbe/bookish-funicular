use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


fn reverse(pair: (i32, bool))->(bool,i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0,matrix.2,matrix.1,matrix.3)
}


impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    //tuple with different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8,-2i16,-3i32,-4i64,
                    0.1f32, 0.2f64, 'a', true);

    //tuple indexing
    println!("long tuple first value : {}", long_tuple.0);
    println!("long tuple second value : {}", long_tuple.1);

    let tuple_of_tuples = ((1u8,2u16,3u32),(4u64,-1i8), -2i16);

    //tuples are printable
    println!("Tuple of tuples : {:?}",tuple_of_tuples);

    // But long Tuples cannot be printed
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}",pair);
    println!("The reversed pair is : {:?}",reverse(pair));

    //one element tuple
    println!("One element tuple : {:?}", (2u32,));
    println!("One element : {:?}", (3u32));

    //tuple destruction binding
    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a,b,c,d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("matrix : {:?}", matrix);
    println!("matrix:\n{}", matrix);
    println!("transponse:\n{}",transpose(matrix));


}