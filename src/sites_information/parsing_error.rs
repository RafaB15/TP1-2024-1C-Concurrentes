/// Enum that represents the possible errors that can occur while parsing the data.
///
/// # Variants
///
/// * `ErrorCouldNotOpenDataDirectory` - The program could not open the data directory.
/// * `ErrorWhileBrowsingTheDirectoryEntries` - An error occurred while browsing the directory entries.
/// * `ErrorWhileBuildingThreadpool` - An error occurred while building the threadpool.
#[derive(Debug)]
pub enum ParsingError {
    ErrorCouldNotOpenDataDirectory,
    ErrorWhileBrowsingTheDirectoryEntries(String),
    ErrorWhileBuildingThreadpool(String),
}
