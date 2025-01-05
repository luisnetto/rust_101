// function to return size of first word of string.
fn first_word(s: &String) -> usize {
    // transform the string into an array of bytes
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()

}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{}", word);
    s.clear(); // This empties the String, making it equal to " "
}
