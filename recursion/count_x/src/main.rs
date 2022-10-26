fn main() {
    let text = "가x나xxxx다x라, 마xx바X사X!";
    // println!("count x : result => {}",count_x(text));
    println!("count x : result => {}",recursive_count_x(text));
    println!("{}",text);
}
 
// fn count_x(text: &str) -> usize {
//     text.chars()
//         .filter(|&char| char == 'x' || char == 'X')
//         .count()
// }

fn recursive_count_x(text: &str) -> usize {
    let mut input = text.to_lowercase();
    match input.len() {
        0 => 0,
        1 if input.contains("x") => 1,
        _ => if input.chars().nth(0).unwrap() == 'x'{
            input.remove(0);
            1+recursive_count_x(&input)
        } else {
            input.remove(0);
            recursive_count_x(&input)
        }
    }
}