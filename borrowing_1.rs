fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Mutable references have one big restriction: if you have a mutable reference to a value
// you can have no other references to that value.

// Example:
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);

// Example 2: 
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM(Combine mutable and immutable references when the scope
// of the immutable references is still active)

// println!("{}, {}, and {}", r1, r2, r3);

// Example of valid code:
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{r3}");