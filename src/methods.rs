use std::fmt;

struct Point {
    x : f64,
    y : f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }
}

struct Rectangle {
    p1 : Point,
    p2 : Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1} = self.p1;
        let Point { x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point{ x: x1, y: y1} = self.p1;
        let Point{ x: x2, y: y2} = self.p2;

        2.0 * ((x1-x2) + (y1- y2)).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.x += y;
        self.p2.y += y;
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Coordinates are ({},{}),({},{})", 
                self.p1.x, self.p1.y,self.p2.x,self.p2.y)
    }
}


struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair ({},{})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0,4.0),
    };

    println!("Rectangle perimeter : {}", rectangle.perimeter());
    println!("Rectangle area : {}", rectangle.area());

    // Rectangle not mutable
    //rectangle.translate(1.0, 0.0);

    let mut square = Rectangle {
        p1 : Point::origin(),
        p2 : Point::new(1.0, 1.0),
    };

    println!("Before translation sqaure is {}", square);
    square.translate(1.0, 1.0);
    println!("After translation sqaure is {}", square);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
 
}