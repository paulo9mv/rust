fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;

    println!("Uma hora equivale a {} segundos", ONE_HOUR_IN_SECONDS);

    let _some_value = 10;
    let some_value = 100;

    {
        let some_value = 999;
        println!("O valor no inner scope é novenovenove: {}", some_value)
    }

    println!("O valor deve ser cem: {}", some_value);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Há {} espaços", spaces);

    let _guess: u32 = "01".parse().expect("Should be a number");

    println!("-------------------------------");

    let a: i16 = 90;
    let b: i16 = -10;

    // Addition
    let sum_a_b = a + b; // 80

    // Minus
    let minus_a_b = a - b; // 100

    // Mult
    let mult = a * b; // -900

    // Division
    let division = a / b; // -9

    println!("{} {} {} {}", sum_a_b, minus_a_b, mult, division);

    let tupla: (f64, u8) = (1.23, 10);

    println!("{}", tupla.0);
    println!("{}", tupla.1);
}
