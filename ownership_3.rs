fn main() {
    let s1 = String::from("hello");
    // Transfer ownership of s1, so s1 is no longer valid.
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}