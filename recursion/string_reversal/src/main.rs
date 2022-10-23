use std::string;

fn main() {
    // println!("Hello, world!");
    let a = "가나다";
    println!("length 가나다 : {}",a.char_indices().count());
    let mut b = a.to_string() + "라";
    println!("length {} : {}",b,b.char_indices().count());
    let c = b.chars().nth(0).unwrap().to_string();
    println!("length {} : {}",b,b.char_indices().count());
    println!("c is {}",c);
    println!("b is {}",b);
    b.remove(0);
    println!("b is {}",b);
    println!("=====================================");
    println!("recursive 나다라 : {}",string_reversal(&mut b));
    // println!("Reverse result : {}",string_reversal(&mut b));
}

fn string_reversal(mut input: &mut String) -> String {
    match input.char_indices().count() {
        1 => input.chars().nth(0).unwrap().to_string(),
        _ => {
            let first_letter = input.remove(0).to_string();
            string_reversal(&mut input) + &first_letter
        }
    }
}   