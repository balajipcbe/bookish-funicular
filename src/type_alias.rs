#[derive(Debug)]
enum VeryVerboseEnumsOfThingsTodoWithNumbers {
    Add,
    Subract
}


enum VeryVerboseOperations {
    Add,
    Subtract
}

impl VeryVerboseOperations {
    fn run(&self, x:i32, y:i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

type Operations = VeryVerboseEnumsOfThingsTodoWithNumbers;

fn main() {
    let x = Operations::Add;
    println!("The Add Operation is encoded to {:?}.",x);
    let y = Operations::Subract;
    println!("The Subtract Operation is encoded to {:?}.",y);
    println!("Add Operations {} ",VeryVerboseOperations::Add.run(5i32, 8i32));
    println!("Subtract Operations {} ",VeryVerboseOperations::Subtract.run(5i32, 8i32));

}