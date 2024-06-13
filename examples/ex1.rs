fn main() {
    // Understanding reduce, fold and scan

    // reduce
    let v = vec![1, 2, 3, 4, 5];
    // Define operation function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    // Use reduce with the defined function
    let reduce_result = v.iter().cloned().reduce(add);
    println!("Reduce result: {:?}", reduce_result.unwrap());

    // fold
    let v = vec![1, 2, 3, 4, 5];
    // Define operation function
    fn other_add(acc: i32, x: &i32) -> i32 {
        acc + x
    }
    // Use fold with the defined function
    let fold_result = v.iter().fold(10, |acc, x| other_add(acc, x));
    println!("Fold result: {:?}", fold_result);

    // scan
    let v = vec![1, 2, 3, 4, 5];
    // Define operation function
    fn oper(a: &mut i32, b: &i32) -> Option<i32> {
        *a = *a + 2 * b;
        Some(*a)
    }
    // Use scan with the defined function
    let mut accumulator = 10;
    let scan_result: Vec<i32> = v
        .iter()
        .scan(&mut accumulator, |acc, x| oper(acc, x))
        .collect();
    println!("Scan result: {:?}", scan_result);
}
