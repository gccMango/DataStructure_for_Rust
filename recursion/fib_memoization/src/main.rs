use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut item_store = HashMap::new();
    fib(6, &mut item_store);
    
    println!("{item_store:?}");
}

fn fib(num: usize, item_store: &mut HashMap<usize, usize>) -> usize {
    if num == 0 || num ==1 {
        item_store.insert(num,1);
        return 1;
    }
    if let None = item_store.get(&num) {
        let v = fib(num-2,item_store) +fib(num-1,item_store);
        item_store.insert(num, v);
    }
    item_store.get(&num).unwrap().to_owned()
}
