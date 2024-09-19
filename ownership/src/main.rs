// fn returns_a_reference() -> &String {
//     let value = String::from("Banana");

//     &value
// }

fn main() {
    println!("Hello, world!");

    let mut some_string = String::from("Paulo Oluap");

    let result = first_word(&some_string);

    println!("The result is {result}");

    some_string.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
