use super::page::Page;

pub struct Pages {
    _pages: Vec<Page>,
}

impl Pages {
    pub fn new(_files_path: &str, _num_threads: u8) -> Self {
        Pages { _pages: Vec::new() }
    }
}