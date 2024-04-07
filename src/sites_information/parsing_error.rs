#[derive(Debug)]
pub enum ParsingError {
    ErrorCouldNotOpenDataDirectory,
    ErrorWhileBrowsingTheDirectoryEntries,
    ErrorWhileBuildingThreadpool(String),
}
