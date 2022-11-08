use std::cmp::Ord;

fn main() {
    let mut vec = vec![-12, -33, 44, 55, 2334, 553, 33256, -55];
    println!("Before : {:?}", vec);
    quick_sort(&mut vec);
    println!("After : {:?}", vec);
    let n = 0;
    println!(
        "quick select : {} th lowest value  => {:?}",
        n + 1,
        quick_select(&mut vec, n)
    );
}
fn quick_select<T: Ord>(arr: &mut [T], kth_lowest_value: isize) -> Option<&T> {
    let len = arr.len();
    let result = quick_select_result(arr, kth_lowest_value, 0, (len - 1) as isize);
    result
}

fn quick_select_result<T: Ord>(
    arr: &mut [T],
    kth_lowest_value: isize,
    left_index: isize,
    right_index: isize,
) -> Option<&T> {
    let index = quick_select_recursive(arr, kth_lowest_value, left_index, right_index);
    arr.get(index as usize)
}

fn quick_select_recursive<T: Ord>(
    arr: &mut [T],
    kth_lowest_value: isize,
    left_index: isize,
    right_index: isize,
) -> isize {
    if right_index <= left_index {
        return left_index;
    } else {
        let p = partition(arr, left_index, right_index);

        if kth_lowest_value < p {
            quick_select_recursive(arr, kth_lowest_value, left_index, p - 1)
        } else if kth_lowest_value > p {
            quick_select_recursive(arr, kth_lowest_value, p + 1, right_index)
        } else {
            p
        }
    }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    quick_sort_recursive(arr, 0, (len - 1) as isize);
}

fn quick_sort_recursive<T: Ord>(arr: &mut [T], left_index: isize, right_index: isize) {
    if left_index < right_index {
        let p = partition(arr, left_index, right_index);
        quick_sort_recursive(arr, left_index, p - 1);
        quick_sort_recursive(arr, p + 1, right_index);
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
