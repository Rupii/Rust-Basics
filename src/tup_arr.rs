pub fn run() {
    let tup: (&str, &str, i32) = ("Rupesh", "CBIT", 21);
    println!("I am {} from {} of age {}", tup.0, tup.1, tup.2);
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", &arr[0..2]);
}
