fn main() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    println!("2 in vec1 {}", vec1.iter() .any(|&x| x == 2));
    println!("2 in vec2 {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("2 in array1 {}", array1.iter().any(|&x| x == 2));
    println!("2 in arrary2 {}", array2.into_iter().any(|&x| x == 2));

    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    //searching 
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    println!("Find 2 in Vec1 {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2 {:?}", into_iter.find(|&x| x == 2));

}