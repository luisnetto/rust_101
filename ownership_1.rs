// Each value in Rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value is dropped

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1= {s1}, s2 = {s2}");
}