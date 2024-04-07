use tp1_fork_join_108225::sites_information::parsing_error::ParsingError;

#[derive(Debug)]
pub enum ErrorExecution {
    WrongAmountOfParameters,
    InvalidNumberOfThreads,
    ErrorInSites(ParsingError),
}
