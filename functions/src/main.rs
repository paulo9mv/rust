fn main() {
    println!("Hello, world!");

    println!("{}", convert_index_to_month_name(1));

    let banana = {
        let maca = 1;
        maca + 9
    };

    println!("{}", banana);
    println!("{}", twenty_six());
    println!("{}", soma_mil(876));
}

fn soma_mil(value: u32) -> u32 {
    // soma mil a qualquer numero
    value + 1000
}

fn twenty_six() -> i8 {
    26
}

fn convert_index_to_month_name(index: usize) -> String {
    let months = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    if index <= 0 || index > months.len() {
        return "Invalid Index".to_string();
    }

    return months[index - 1].to_string();
}
