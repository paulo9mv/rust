fn main() {
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
