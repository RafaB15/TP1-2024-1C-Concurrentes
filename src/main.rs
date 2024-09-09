mod error_execution;

use error_execution::ErrorExecution;
use serde_json::Value;
use std::env::args;
use tp1_fork_join_108225::sites_information::sites_collection::SitesCollection;

const DATA_PATH: &str = "test_data";
const PADRON: &str = "108225";
const ARG_AMOUNT: usize = 2;

/// Verifies the amount of arguments provided to the program. If the amount is invalid, it returns an error.
///
/// # Arguments
///
/// * `arguments` - A vector of strings containing the arguments provided to the program.
///
/// # Returns
///
/// A result that could carry an ErrorExecution.
fn verify_amount_of_arguments(arguments: &[String]) -> Result<(), ErrorExecution> {
    if arguments.len() != ARG_AMOUNT {
        eprintln!("Wonrg number of arguments provided. The program should be executed with cargo run <number_of_working_threads>");
        return Err(ErrorExecution::WrongAmountOfParameters);
    }
    Ok(())
}

/// Returns the number of threads to be used by the program.
/// If the number is invalid, it returns an error.
///
/// # Arguments
///
/// * `number_of_threads` - A string containing the number of threads to be used by the program.
///
/// # Returns
///
/// A result with the number of threads to be used by the program.
fn obtain_number_worker_threads(number_of_threads: &str) -> Result<u8, ErrorExecution> {
    match number_of_threads.parse::<u8>() {
        Ok(number) => Ok(number),
        Err(_) => Err(ErrorExecution::InvalidNumberOfThreads),
    }
}

/// Main function of the program.
/// It receives the number of threads to be used by the program as an argument in the command line.
/// It loads the sites from the file "test_data" and generates a JSON report with the information of the sites.
/// It prints the report in stdout.
fn main() -> Result<(), ErrorExecution> {
    let arguments: Vec<String> = args().collect();
    verify_amount_of_arguments(&arguments)?;
    let num_threads = obtain_number_worker_threads(&arguments[1])?;

    let mut sites = SitesCollection::new();

    if let Err(er) = sites.set_thread_amount(num_threads) {
        return Err(ErrorExecution::ErrorInSites(er));
    };

    if let Err(er) = sites.load_sites(DATA_PATH) {
        return Err(ErrorExecution::ErrorInSites(er));
    };

    let parsed: Value = sites.generate_json_information(PADRON);

    let string_report = match serde_json::to_string_pretty(&parsed) {
        Ok(report) => report,
        Err(_) => return Err(ErrorExecution::ErrorInJsonParsing),
    };

    println!("{}", string_report);

    Ok(())
}
