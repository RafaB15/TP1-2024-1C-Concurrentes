use std::env::args;

fn main() {
    let arguments: Vec<String> = args().collect();

    if arguments.len() != 2 {
        println!("Wonrg number of arguments provided. The program should be executed with cargo run <number_of_working_threads>");
        return;
    }

    let worker_threads = match arguments[1].parse::<u8>() {
        Ok(number) => number,
        Err(_) => {
            print!("The specified number of workers isn't a valid number. Please enter a different one");
            return;
        }
    };
}
