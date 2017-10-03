use std::io;

// 0: Start Menu
// 1: Farhenheit to Celcius
// 2: Celcius to Fahrenheit

const CONTEXT_LIST: [i8; 3] = [0, 1, 2];

fn main() {
    let context = 0;
    let to_continue = true;
    let mut state = State { context, to_continue };

    println!("Welcome to the Fahrenheit/Celcius Converter");
    while state.to_continue {
        let mut input = String::new();

        print_options(state.context);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        state = process_input(input, state);
    }
    println!("Bye-Bye!");
}

struct State {
    context: i8,
    to_continue: bool
}

fn print_options(current_ctx: i8) {
    match current_ctx {
        0 => println!("1 -> Fahrenheit to Celcius\n2 -> Celcius to Fahrenheit\nq -> Quit"),
        1 => println!("Please enter your Fahrenheit temperature:"),
        2 => println!("Please enter your Celcius temperature:"),
        _ => println!("Not recognized and quitting"),
    }
}

fn process_input(input: String, state: State) -> State {
    // Quit
    if input == "q" {
        State { context: 0, to_continue: false }
    // Otherwise, we're continuing
    } else {
        // In Start-Menu
        if state.context == 0 {
            let new_context: i8 = match input.parse() {
                Ok(num) => num,
                Err(_) => 0
            };
            State { context: new_context, to_continue: true }
        } else {
            // We're in Convertion Mode & input is to b converted
            // Parse the temperature value
            let to_convert: f64 = match input.parse() {
                Ok(num) => num,
                Err(_) => 0f64
            };
            // If unequal to 0, no error
            if to_convert != 0f64 {
                let converted_temp = convertTemperature(to_convert, state.context);

                println!("{}", to_convert);
            }
            State { context: 0, to_continue: true }
        }
    }
}

fn getTempSymbols(context: i8) -> [String;2] {
    let mut DEFAULT_SYMBOLS = [String::from("°F"),String::from("°C")];
    if context == 1 {
        DEFAULT_SYMBOLS
    } else {
        DEFAULT_SYMBOLS.iter().reverse()
    }
}

fn convertTemperature(temperature: f64, context: i8) -> f64 {
    if context == 1 {
        fahenheitToCelcius(temperature)
    } else {
        celciusToFahenheit(temperature)
    }
}

fn fahenheitToCelcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32f64) * (5f64/9f64)
}

fn celciusToFahenheit(celcius: f64) -> f64 {
    celcius * (9f64/5f64) + 32f64
}