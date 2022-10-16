fn is_subset<T: PartialOrd>(v1: &[T], v2: &[T]) -> bool {
    let small_vec;
    let large_vec;
    if v1.len() > v2.len() {
        small_vec = v2;
        large_vec = v1;
    } else {
        small_vec = v1;
        large_vec = v2;
    }
    for i in 0..small_vec.len() {
        let mut found_match = false;

        for j in 0..large_vec.len() {
            if small_vec[i] == large_vec[j] {
                found_match = true;
                break;
            }
        }

        if found_match == false {
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
