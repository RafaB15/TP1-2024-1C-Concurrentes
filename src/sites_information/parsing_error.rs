/**
 * Represents the possible errors that can occur while parsing the data.
 */
#[derive(Debug)]
pub enum ParsingError {
    ErrorCouldNotOpenDataDirectory,
    ErrorWhileBrowsingTheDirectoryEntries(String),
    ErrorWhileBuildingThreadpool(String),
}
