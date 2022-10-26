fn main() {
    println!("stair case all of case result :{}", number_of_paths(5));
}

fn number_of_paths(num: isize) -> usize {
    if num < 0 {
        return 0;
    }
    match num {
        0 | 1 => 1,
        _ => number_of_paths(num - 1) + number_of_paths(num - 2) + number_of_paths(num - 3),
    }
}
