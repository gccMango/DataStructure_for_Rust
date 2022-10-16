fn bubble_sort(vec: &mut Vec<i32>) {
    let mut sorted_flag = false;
    let mut unsorted_index = vec.len() - 1;
    while !sorted_flag {
        sorted_flag = true;
        for index in 0..unsorted_index {
            if vec[index] > vec[index + 1] {
                vec.swap(index, index + 1);
                sorted_flag = false;
            }
        }
        unsorted_index-=1;
    }
}

fn main() {
    let mut vec = vec![-1, 3, -55, 30, -42, 55, -13,4,22];
    println!("vec 0st sorted {:?}", vec);

    bubble_sort(&mut vec);

    println!("vec sorted {:?}", vec);
}
