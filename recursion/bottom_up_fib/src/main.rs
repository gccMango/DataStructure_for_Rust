fn main() {
    println!("Hello, world!");
    println!("{} : fibonacci : {}",6, fib_bottom_up(6))
}
fn fib_bottom_up(num: usize)->usize {
    if num == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;

    for i in 1..num {
        let mut temp = a;
        a = b;
        b = temp +a;
    }
    b
}