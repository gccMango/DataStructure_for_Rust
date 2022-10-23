fn main() {
    println!("Hello, world!");

    let mut x = [1, 2, 3, 4, 5];
    println!("{:?}", x);
    double_array(&mut x, 3);
    println!("{:?}", x);
}

fn double_array(array: &mut [i32], index: usize) {
    match index {
        index if index < array.len() => {
            println!("doubling at index {}",index);
            array[index]*=2;
            double_array(array, index+1)
        },
        _ => {
            println!("Stop recursion");
        }
    }
}
