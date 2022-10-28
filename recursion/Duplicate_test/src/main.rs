fn main() {
    println!("Hello, world!");
    let mut arr = vec![1, 2, 4, 3, 3, 4, 5];
    println!("result {}", duplicate_test(&mut arr));
}

fn duplicate_test<T: Ord + PartialEq>(arr: &mut [T]) -> bool {
    let mut flag = false;

    arr.sort();
    for i in 0..arr.len(){
        if i + 1 <= arr.len() - 1 {
            if arr[i] == arr[i + 1] {
                flag = true;
                break;
            }
        }

    }

    flag
}
