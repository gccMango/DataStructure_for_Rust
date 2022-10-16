fn insert_sort(vec: &mut Vec<i32>) {
    for index in 1..vec.len() {
        let mut tmp_val = vec[index];

        let mut position = index - 1;

        while vec[position] > tmp_val {
            vec[position + 1] = vec[position];
            if position == 0 {
                break;
            }
            position-=1;
        }

        if position == 0 && vec[0] >tmp_val {
            vec[0] = tmp_val;
        } else{
            vec[position+1] = tmp_val;
        }
    }
}

fn main() {
    let mut vec = vec![-2, 3, -11, -27, 33];
    insert_sort(&mut vec);
    println!("after sorting : {:?}", vec);
}
