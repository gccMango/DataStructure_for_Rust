fn main() {
    println!("Hello, world!");
    println!("factorial result is {}",factorial(5));
}

fn factorial(num: u8)-> u8 {
    println!("factorial number {num}");
    if num == 1 {
        return 1
    } else {
        return num * factorial(num -1)
    }
}