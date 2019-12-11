#[derive(Debug)]
struct Person<'a> {
    name : &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A Struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1 : Point,
    p2 : Point,
}

fn rect_area(rectangle:Rectangle) {
    let Rectangle{
         p1: Point { x: x1, y: y1 },
         p2: Point { x: x2, y: y2 },
    } = rectangle;


    println!("Rectangle area : {}", (x1-x2) *(y1-y2));
}

fn square(point: Point, offset: f32) -> Rectangle {
    let x_offset = point.x + offset;
    let y_offset = point.y + offset;

    let rect = Rectangle {
        p1: point,
        p2: Point { x:x_offset, y:y_offset}
    };
    return rect;
}

fn main() {
    let name = "Peter";
    let age = 28;
    let peter = Person { name, age};

    println!("{:?}", peter);

    //Instantiate a Point 
    let point: Point = Point { x: 0.3, y: 0.5};

    println!("point Coordinates: ({},{})", point.x, point.y);

    let new_point = Point{ x: 0.1, ..point };
    println!("second point: ({},{})", new_point.x, new_point.y);

    let Point {x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1 : Point { x: my_y, y: my_x},
        p2 : point,
    };

    println!("Rectangle point coordinates ({},{}), ({},{})",
                _rectangle.p1.x,
                _rectangle.p1.y,
                _rectangle.p2.x,
                _rectangle.p2.y);

    let _nil = Nil;

    let pair = Pair(1,0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    rect_area(_rectangle);

    let point2 : Point = Point { x: 1.1, y: 2.3};
    let  the_rect = square(point2, 2.3);

    println!("{:?}", the_rect);
}