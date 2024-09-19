fn get_value(value: String) -> String {
    println!("{}", value);

    return value;
}

fn main() {
    let literal_string = "Oi sou literal";
    let not_literal_string = String::from(literal_string);
    let mut other = String::from(literal_string);

    let _banana = not_literal_string.clone();

    println!("{not_literal_string}");

    other = get_value(other);
    println!("{}", other);

    println!("Hello, world!");

    let number = 10;

    if number < 5 {
        println!("Número menor que 5");
    } else {
        println!("Número nao menor que 5");
    }

    let numero_menor_que_cinco: bool = number < 5;

    let algo = if numero_menor_que_cinco { "<" } else { ">" };

    println!("{}", algo);
}
