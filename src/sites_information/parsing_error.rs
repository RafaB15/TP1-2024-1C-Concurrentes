#[derive(Debug)]
pub enum ParsingError {
    ErrorCouldNotOpenDataDirectory,
    ErrorWhileBrowsingTheDirectoryEntries(String),
    ErrorWhileBuildingThreadpool(String),
}
