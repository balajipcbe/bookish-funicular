fn eat_box_i32(boxed_i32: Box<i32>)
{
    println!("Destorying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32)
{
    println!("This int is : {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5i32);
    let stacked_i32 = 100i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32:&i32 = &boxed_i32;

        //eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}