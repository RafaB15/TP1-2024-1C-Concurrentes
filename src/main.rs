mod error_execution;

use error_execution::ErrorExecution;
use serde_json::Value;
use std::env::args;
use tp1_fork_join_108225::sites_information::sites_collection::SitesCollection;

const DATA_PATH: &str = "test_data";

fn verify_amount_of_arguments(arguments: &Vec<String>) -> Result<(), ErrorExecution> {
    if arguments.len() != 2 {
        eprintln!("Wonrg number of arguments provided. The program should be executed with cargo run <number_of_working_threads>");
        return Err(ErrorExecution::WrongAmountOfParameters);
    }
    Ok(())
}

fn obtain_number_worker_threads(number_of_threads: &String) -> Result<u8, ErrorExecution> {
    match number_of_threads.parse::<u8>() {
        Ok(number) => Ok(number),
        Err(_) => Err(ErrorExecution::InvalidNumberOfThreads),
    }
}

fn main() -> Result<(), ErrorExecution> {
    let arguments: Vec<String> = args().collect();
    verify_amount_of_arguments(&arguments)?;
    let num_threads = obtain_number_worker_threads(&arguments[1])?;

    let start = std::time::Instant::now();
    let mut sites = SitesCollection::new();

    if let Err(er) = sites.load_sites(DATA_PATH, num_threads) {
        return Err(ErrorExecution::ErrorInSites(er));
    };

    let finish = std::time::Instant::now();
    let duration = finish.duration_since(start);

    sites.print_info();

    println!("Duró {}\n", duration.as_millis());

    let parsed: Value = sites.generate_json_information("108225");
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}
