fn main() {

    let s1 = String::from("Hello wolrd");

    let first_str = first_string(&s1);

    println!("{first_str}");

    // s1.push_str(", world!");

    // // let s2 = s1.clone();

    
    // // println!("{s1}");

    // change(&mut s1);
    // let len = calculate_length(&s1);

    // println!("{len}");
    // takes_ownership(s1);
    // let x = 5;
    
    // makes_copy(x);
    // let y = x;
    // println!("{y}");



    // println!("{y}");
    // println!("{x}");
    // println!("s1 + s2 Agora funcionam: {s1},{s2}");
}


// fn takes_ownership(some_string: String){
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32){
//     println!("{some_integer}");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String){
//     some_string.push_str(", world");
// }


fn first_string(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

