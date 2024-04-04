#[derive(Debug)]
pub enum ParsingError {
    ErrorCouldNotOpenDataDirectory,
    ErrorWhileBrowsingTheDirectoryEntries,
}