fn main() {
    
    countdown_rocket(10);
}

fn countdown_rocket(num: i8) {
    if num == 0 {
        return println!("Fire!")
    }
    println!("countdown rocket : {}",num);
    countdown_rocket(num-1)
}