fn first_word(s: &String) ->  &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("Hello World");
    let word = first_word(&s);

    // s.clear(); if we clear s before use var word, we will have compilation error.

    println!("{}", word); 

    s.clear();

    /* if we clear after, immutable borrow will not be used anymore, so we will 
    not have compilation error. */ 

    

}