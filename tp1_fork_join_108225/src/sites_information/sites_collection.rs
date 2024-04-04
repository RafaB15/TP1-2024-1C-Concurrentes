use super::{
    parsing_error::ParsingError, question::Question, question_information::QuestionInformation, site::Site
};

use std::{
    fs::{read_dir, File, ReadDir}, io::{BufReader, BufRead}, path::PathBuf};

use rayon::prelude::*;

#[derive(Debug)]
pub struct SitesCollection {
    sites: Vec<Site>,
}

const JSONL_EXTENSION: &str = "jsonl";

impl SitesCollection {
    pub fn new(files_path: &str) -> Result<Self, ParsingError> {
        let data_directory = Self::get_directory(files_path)?;
        let files_paths = Self::get_files_paths(data_directory)?;
        println!("{:?}\n", files_paths);
        let sites = Self::get_sites(files_paths);

        Ok(SitesCollection { sites })
    }

    fn get_directory(files_path: &str) -> Result<ReadDir, ParsingError>{
        match read_dir(files_path) {
            Ok(directory) => Ok(directory),
            Err(_) => return Err(ParsingError::ErrorCouldNotOpenDataDirectory),
        }
    }

    fn get_files_paths(directory: ReadDir) -> Result<Vec<PathBuf>, ParsingError>{
        let dir_entries: Vec<PathBuf> = directory
                        .filter_map(|entry| {
                            let valid_entry =  match entry {
                                Ok(valid_entry) => valid_entry,
                                Err(_) => return None,
                            };
                            let path = valid_entry.path();
                            if let Some(extension) = path.extension() {
                                if extension == JSONL_EXTENSION {
                                    return Some(path)
                                }
                            }
                            None
                        }).collect();
        if dir_entries.len() == 0 {
            return Err(ParsingError::ErrorWhileBrowsingTheDirectoryEntries);
        }
        Ok(dir_entries)
    }

    fn get_sites(files_paths: Vec<PathBuf>) -> Vec<Site> {
        let sites: Vec<Site> = files_paths.par_iter().filter_map(|path| {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                let questions_info: Vec<QuestionInformation> = reader.lines().par_bridge().filter_map(|line| {
                    let valid_line = match line {
                        Ok(line) => line,
                        Err(_) => return None,
                    };
                    match serde_json::from_str::<Question>(&valid_line){
                        Ok(question) => Some(question.into_information()),
                        Err(e) => {
                            println!("{}", e);
                            None
                        },
                    }
                }).collect();
                Some(Site::new(questions_info))
            } else {
                None
            }
        }).collect();

        sites
    }

    pub fn print_info(&self) {
        for site in self.sites.iter() {
            site.print_info();
        }
    }
}
