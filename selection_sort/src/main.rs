use std::cmp::PartialOrd;


fn selection_sort<T: PartialOrd>(vec: &mut[T])  {
    for i in 0..vec.len() {
        let mut lower_num_index = i;
        for j in (i+1)..vec.len() {
            if vec[j]< vec[lower_num_index] {
                lower_num_index = j;
            }
        }
        if lower_num_index != i {
            vec.swap(i, lower_num_index);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic() {
        let mut vec = vec![1,22,44,-22,33,499,-19];
        selection_sort(&mut vec);
        println!("After sorting {:?}",vec);
        
        assert_eq!(vec, vec![-22,-19,1,22,33,44,499]);
    }
    
    #[test]
    fn basic_char() {
        let mut vec = vec!['z','x','A','w','Q'];
        selection_sort(&mut vec);
        println!("After sorting {:?}",vec);
    
        assert_eq!(vec, vec!['A','Q','w','x','z']);
        
    }
}
