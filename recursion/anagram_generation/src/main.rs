fn main() {
    // let input_chars = vec!["a", "b", "c"];

    // let combinations = input_chars
    //     .iter()
    //     .map(|&c| input_chars.iter().map(move |&d| d.to_owned() + c))
    //     .flatten()
    //     .collect::<Vec<_>>();
    // let text ="hello";
    // text.chars().last();
    // println!("{}",text.chars().nth(0).unwrap());
    // println!("{}",text);
    // println!("{:?}",combinations);

    println!("{:?}",anagram_generator("abc"));
}
fn anagram_generator(text: &str) -> Vec<String> {
    if text.len() ==1 {
        return vec![text.to_string()];
    }
    let mut vec_str = vec![];
    let mut input = text.to_string();
    let letter =input.remove(0);
    let mut substring_anangrams = anagram_generator(input.as_str());
    substring_anangrams.into_iter().for_each(|mut item| {
        (0..item.len()).for_each(|x|{
            item.insert(x, letter);
            vec_str.push(item.clone());
        })
    });
    vec_str
}
