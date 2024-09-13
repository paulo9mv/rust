fn main() {
    println!("Hello, world!");

    let converted_fah: f64 = f_to_c(123.0); // 50.556
    let converted_cel: f64 = c_to_f(50.555556); // 50.556

    println!("123 Fahrenheit equivale a {} Celsius", converted_fah);
    println!("50.555556 Celsius equivale a {} Fahrenheit", converted_cel);
}

fn f_to_c(fahrenheit: f64) -> f64 {
    let celsius: f64 = (fahrenheit - 32.0) * (5.0 / 9.0);

    celsius
}

fn c_to_f(celsius: f64) -> f64 {
    let fahrenheit: f64 = 1.8 * celsius + 32.0;

    fahrenheit
}
