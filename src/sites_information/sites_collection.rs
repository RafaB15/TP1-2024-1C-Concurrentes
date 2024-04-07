use super::{parsing_error::ParsingError, question::Question, site::Site};

use std::{
    fs::{read_dir, File, ReadDir},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use rayon::{prelude::*, ThreadPoolBuilder};

#[derive(Debug)]
pub struct SitesCollection {
    sites: Option<Vec<Site>>,
}

const JSONL_EXTENSION: &str = "jsonl";

impl SitesCollection {
    pub fn new() -> Self {
        SitesCollection { sites: None }
    }

    pub fn load_sites(&mut self, files_path: &str, num_threads: u8) -> Result<(), ParsingError> {
        let data_directory = Self::get_directory(files_path)?;
        let files_paths = Self::get_files_paths(data_directory)?;
        println!("{:?}\n", files_paths);
        match Self::get_sites(files_paths, num_threads) {
            Ok(sites) => self.sites = Some(sites),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn get_directory(files_path: &str) -> Result<ReadDir, ParsingError> {
        match read_dir(files_path) {
            Ok(directory) => Ok(directory),
            Err(_) => return Err(ParsingError::ErrorCouldNotOpenDataDirectory),
        }
    }

    fn get_files_paths(directory: ReadDir) -> Result<Vec<PathBuf>, ParsingError> {
        let dir_entries: Vec<PathBuf> = directory
            .filter_map(|entry| {
                let valid_entry = match entry {
                    Ok(valid_entry) => valid_entry,
                    Err(_) => return None,
                };
                let path = valid_entry.path();
                if let Some(extension) = path.extension() {
                    if extension == JSONL_EXTENSION {
                        return Some(path);
                    }
                }
                None
            })
            .collect();
        if dir_entries.len() == 0 {
            return Err(ParsingError::ErrorWhileBrowsingTheDirectoryEntries);
        }
        Ok(dir_entries)
    }

    fn get_sites(files_paths: Vec<PathBuf>, num_threads: u8) -> Result<Vec<Site>, ParsingError> {
        Self::set_thread_amount(num_threads)?;
        let sites: Vec<Site> = files_paths
            .par_iter()
            .filter_map(|path| {
                if let Ok(file) = File::open(&path) {
                    let reader = BufReader::new(file);
                    let site = reader
                        .lines()
                        .par_bridge()
                        .filter_map(|line| {
                            let valid_line = match line {
                                Ok(line) => line,
                                Err(_) => return None,
                            };
                            match serde_json::from_str::<Question>(&valid_line) {
                                Ok(question) => Some(question.into_information()),
                                Err(e) => {
                                    println!("{}", e);
                                    None
                                }
                            }
                        })
                        .fold(
                            || Site::new(),
                            |mut site, question| {
                                site.add_question(question);
                                site
                            },
                        )
                        .reduce(Site::new, |mut site, other| {
                            site.merge(other);
                            site
                        });
                    Some(site)
                } else {
                    None
                }
            })
            .collect();

        Ok(sites)
    }

    pub fn print_info(&self) {
        if let Some(sites) = &self.sites {
            for site in sites.iter() {
                site.print_info();
            }
        }
    }

    fn set_thread_amount(num_threads: u8) -> Result<(), ParsingError> {
        if let Err(error) = ThreadPoolBuilder::new()
            .num_threads(num_threads as usize)
            .build_global()
        {
            return Err(ParsingError::ErrorWhileBuildingThreadpool(
                error.to_string(),
            ));
        }
        Ok(())
    }
}
