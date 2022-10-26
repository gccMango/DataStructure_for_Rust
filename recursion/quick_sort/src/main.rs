use std::cmp::Ord;

fn main() {
    let mut vec = vec![-12, -33, 44, 55, 2334, 553, 33256, -55];
    println!("Before : {vec:?}");

    // println!("result lo : {}", partition(&mut vec, 0, hi));
    quick_sort(&mut vec);
    println!("partition After : {vec:?}");
}
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    quick_sort_recursive(arr, 0, (len - 1) as isize);
}

fn quick_sort_recursive<T: Ord>(arr: &mut [T], left_index: isize, right_index: isize) {
    if left_index < right_index {
        let p = partition(arr, left_index, right_index);
        quick_sort_recursive(arr, left_index, p-1);
        quick_sort_recursive(arr, p+1, right_index);
    }
}
fn partition<T: Ord>(arr: &mut [T], mut lo: isize, mut hi: isize) -> isize {

    let pivot_index = hi as usize;

    hi -= 1;

    loop {
        while arr[lo as usize] < arr[pivot_index] {
            lo += 1;
        }
        while hi > 0 && (arr[hi as usize] > arr[pivot_index]) {
            hi -= 1;
        }

        if lo >= hi {
            break;
        } else {
            arr.swap(lo as usize, hi as usize);
            lo += 1;
        }
    }
    arr.swap(lo as usize, pivot_index);
    lo
}
