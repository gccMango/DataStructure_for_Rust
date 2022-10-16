use std::collections::HashMap;
use std::hash::Hash;

fn is_subset<T: PartialOrd +Hash +Eq>(v1: &[T], v2: &[T]) -> bool {
    let large_vec;
    let small_vec;
    if v1.len() > v2.len() {
        small_vec = v2;
        large_vec = v1;
    } else {
        small_vec = v1;
        large_vec = v2;
    }
    
    let hash_map:HashMap<_,_> = large_vec.iter().map(|x| (x,true)).collect();

    for item in small_vec {
        if let None = hash_map.get(item){
            return false;
        }
    }

    return true;
}
    
fn main() {
    let v1 = vec!["a", "b", "c", "d", "e", "f"];
    // let v2 = vec!["b", "d", "h", "f"];
    let v2 = vec!["b", "d", "f"];

    println!("v1 and v2 is subset? {}", is_subset(&v1, &v2));
}
