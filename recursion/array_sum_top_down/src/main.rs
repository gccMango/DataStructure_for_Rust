fn main() {
    // let arr = [-1,2];
    let arr = [-1,4,3,-11,33,-123];
    println!("we starting Top-down recursion sum");
    println!("result : {}",sum(&arr));
}

fn sum(array: &[isize]) -> isize {
    match array.len() {
        1 => array[0],
        _ => array[0]+sum(&array[1..array.len()])
    }
}