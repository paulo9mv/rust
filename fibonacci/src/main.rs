fn main() {
    println!("Hello, world!");

    println!("{}", fibonacci(10));
}

fn fibonacci(limit: u32) -> u32 {
    let mut value = 1;
    let mut anterior = 0;
    let mut mutable_limit = limit;

    if limit == 0 {
        return value;
    }

    while mutable_limit > 0 {
        mutable_limit -= 1;

        let tmp = value;

        value += anterior;
        anterior = tmp;

        println!("{value}");
    }

    return value;
}
