fn main() {
    println!("Hello, world!");
    let mut counter = 0;
    let mut counter2 = 0;

    // let meu_loop = loop {
    //     println!("Oi, {counter}");

    //     if counter > 100 {
    //         break "P";
    //     }

    //     counter += 1;
    // };

    //println!("Resultado do loop {}", meu_loop);

    'loop_exterior: loop {
        println!("Oi, {counter}"); // Só sera printado 1x

        if counter > 100 {
            break;
        }

        counter += 1;

        'loop_interior: loop {
            counter2 += 1;
            println!("Sou o {counter2}");

            if counter2 > 5 {
                counter2 = 0;
                break 'loop_exterior;
            }
        }
    }

    let mut count_while = 0;

    while count_while < 5 {
        count_while += 1;
        println!("{count_while}");
    }

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

    for month in months {
        println!("Sou o mês: {}", month);
    }

    for value in (1..100).rev() {
        print!("{value} ");
    }
}
