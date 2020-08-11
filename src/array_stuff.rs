pub fn array_test() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first element is: {}", first);

    for i in 0..arr.len() { //better than example given which asked to do 0 to 4 manually
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length of arr is {}", arr.len());
}