fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// &String is a reference to a String and it allows you to refer to some value without taking ownership of it.
fn calculate_length(s: &String) -> usize {
    s.len()
}