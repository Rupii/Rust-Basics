pub fn run() {
    let hello = "Hello "; // immutable string
    let mut hell = String::from("Hello ");
    hell.push('w');
    hell.push_str("orld");
    println!("{}", hell);
    println!(" {} contailns word Hello", hell.contains("Hello"));
    println!("replace {}", hell.replace("world", "there"));
    for word in hell.split_whitespace() {
        println!("{}", word);
    }
    let mut s = String::with_capacity(10);
    s.push('h');
}
