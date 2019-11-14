//importing fmt module
use std::fmt;

// struct with single element
#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    //trait requires exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write the first element in output stream f
        write!(f,"{}", self.0)
    }
}

#[derive(Debug)]
// struct with double elements
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({},{})", self.0, self.1)
    }
}
#[derive(Debug)]
// struct with named elements
struct Point2D {
    x : f64,
    y : f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f,"(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
// struct with named elements
struct Complex {
    x : f64,
    y : f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f,"{{real:{} + imag:{}i}}", self.x, self.y)
    }
}

fn main() {
    // single item struct
    let aitem = Structure(993);
    println!("Display:{}",aitem);
    println!("Debug:{:?}",aitem);

    // double item struct
    let minmax = MinMax(0,14);
    println!("Compare Structures");
    println!("Display:{}", minmax);
    println!("Debug:{:?}",minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    //named item struct
    let point = Point2D{ x: 3.3, y: 3.8};
    println!("Compare Points");
    println!("Display:{}",point);
    println!("Debug:{:?}", point);

    //Complex number display
    let item = Complex{x:1.3,y:9.2};
    println!("Complex Number");
    println!("Display:{}",item);
    println!("Debug:{:?}",item);

}
