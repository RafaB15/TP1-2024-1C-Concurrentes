mod error_initialization;

use std::env::args;
use error_initialization::ErrorInitialization;
use serde_json::Value;
use tp1_fork_join_108225::sites_information::sites_collection::SitesCollection;

const DATA_PATH: &str = "data";

fn verify_amount_of_arguments(arguments: &Vec<String>) -> Result<(), ErrorInitialization>{
    if arguments.len() != 2 {
        eprintln!("Wonrg number of arguments provided. The program should be executed with cargo run <number_of_working_threads>");
        return Err(ErrorInitialization::WrongAmountOfParameters)
    }
    Ok(())
}

fn obtain_number_worker_threads(number_of_threads: &String) -> Result<u8, ErrorInitialization> {
    match number_of_threads.parse::<u8>() {
        Ok(number) => Ok(number),
        Err(_) => Err(ErrorInitialization::InvalidNumberOfThreads)
    }
}

fn main() -> Result<(), ErrorInitialization> {
    let arguments: Vec<String> = args().collect();
    verify_amount_of_arguments(&arguments)?;
    let _num_threads = obtain_number_worker_threads(&arguments[1])?;

    let start = std::time::Instant::now();

    let sites = match SitesCollection::new(DATA_PATH){
        Ok(sites) => sites,
        Err(er) => return Err(ErrorInitialization::ErrorInSites(er)),
    };

    let finish = std::time::Instant::now();
    let duration = finish.duration_since(start);

    sites.print_info();

    println!("Duró {}\n", duration.as_millis());
    let answer = r#"{"padron": "108225"}"#;
    let parsed: Value = serde_json::from_str(answer).unwrap();
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}
