fn has_duplicate_value(vec: &mut Vec<i32>)->bool{
    let mut count = 0;
    for i in 0..vec.len() {
        for j in 0..vec.len(){
            count +=1;
            if i!=j && vec[i] == vec[j]{
                println!("count : {}",count);
                return true
            }
        }
    }
    println!("count : {}",count);
    false
}

fn main() {
    let mut vec = vec![1,2,3,44,55,11,3];

    println!("result {}",has_duplicate_value(&mut vec));
}
