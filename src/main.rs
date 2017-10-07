use std::io;
use std::str::FromStr;
use std::num::ParseIntError;
// use
// 0: Start Menu
// 1: Farhenheit to Celcius
// 2: Celcius to Fahrenheit

// const CONTEXT_LIST: [u8; 3] = [0, 1, 2];

fn main() {
    let context = 0u8;
    let to_continue = true;
    let mut state = State { context, to_continue };

    println!("Welcome to the Fahrenheit/Celcius Converter");
    while state.to_continue {
        let mut input = String::new();

        print_options(state.context);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        state = process_input(&mut input, state);
    }
    println!("Bye-Bye!");
}

struct State {
    context: u8,
    to_continue: bool
}

fn print_options(current_ctx: u8) {
    match current_ctx {
        0u8 => println!("1 -> Fahrenheit to Celcius\n2 -> Celcius to Fahrenheit\nq -> Quit"),
        1u8 => println!("Please enter your Fahrenheit temperature:"),
        2u8 => println!("Please enter your Celcius temperature:"),
        _ => println!("Not recognized and quitting"),
    }
}

fn process_input(input: &mut String, state: State) -> State {
    // match input.as_ref() {
    match input.trim() {
        "q" => {
            State { context: 0u8, to_continue: false }
        },
        _ => {
            // In Start-Menu
            if state.context == 0u8 {
                let new_context = match input.trim().parse::<u8>() {
                    Ok(num) => num,
                    Err(error) => {
                        println!("{}", error);
                        0
                    }
                };
                State { context: new_context, to_continue: true }
            } else {
                // We're in Convertion Mode & input is to b converted
                // Parse the temperature value
                let to_convert = match input.trim().parse::<f64>() {
                    Ok(num) => num,
                    Err(_) => 0f64
                };
                // If unequal to 0, no error
                if to_convert != 0f64 {
                    let converted_temp = convert_temperature(to_convert, state.context);
                    let temp_symbols = get_temp_symbols(state.context);

                    println!("{}{} is equivalent to {}{}.", to_convert, temp_symbols[0], converted_temp, temp_symbols[1]);
                }
                State { context: 0, to_continue: true }
            }
        },
    }
}

fn get_temp_symbols(context: u8) -> Vec<String> {
    let mut default_symbols: Vec<String> = vec![String::from("°F"), String::from("°C")];
    if context == 1u8 {
        default_symbols
    } else {
        default_symbols.reverse();
        default_symbols
    }
}

fn convert_temperature(temperature: f64, context: u8) -> f64 {
    if context == 1u8 {
        fahenheit_to_celcius(temperature)
    } else {
        celcius_to_fahenheit(temperature)
    }
}

fn fahenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32f64) * (5f64/9f64)
}

fn celcius_to_fahenheit(celcius: f64) -> f64 {
    celcius * (9f64/5f64) + 32f64
}