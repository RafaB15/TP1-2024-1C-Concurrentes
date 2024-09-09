use tp1_fork_join_108225::sites_information::parsing_error::ParsingError;

/// Enum that represents the possible errors that can occur during the execution of the program.
///
/// # Variants
///
/// * `WrongAmountOfParameters` - The amount of parameters provided to the program is invalid.
/// * `InvalidNumberOfThreads` - The number of threads provided to the program is invalid.
/// * `ErrorInSites` - An error occurred while loading the sites.
/// * `ErrorInJsonParsing` - An error occurred while parsing the JSON.
#[derive(Debug)]
pub enum ErrorExecution {
    WrongAmountOfParameters,
    InvalidNumberOfThreads,
    ErrorInSites(ParsingError),
    ErrorInJsonParsing,
}
