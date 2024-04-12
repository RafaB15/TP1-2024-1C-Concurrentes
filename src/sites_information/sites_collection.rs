use super::{
    parsing_error::ParsingError, question::Question, site::Site, tags_collection::TagsCollection,
};

use std::{
    fs::{read_dir, File, ReadDir},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use serde_json::Value;

use rayon::{prelude::*, ThreadPoolBuilder};

#[derive(Debug)]
pub struct SitesCollection {
    sites: Option<Vec<Site>>,
}

const JSONL_EXTENSION: &str = "jsonl";

impl SitesCollection {
    /**
     * Creates a new SitesCollection instance.
     */
    pub fn new() -> Self {
        SitesCollection { sites: None }
    }

    /**
     * Sets the amount of threads to be used by the program.
     * If there was an error while building the threadpool, it returns an error.
     */
    pub fn set_thread_amount(&self, num_threads: u8) -> Result<(), ParsingError> {
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

    /**
     * Loads the sites with json extension from the provided path using the provided amount of threads.
     * The function returns an error if the path is invalid, if the directory is empty or if there was an error while reading the files.
     */
    pub fn load_sites(&mut self, files_path: &str) -> Result<(), ParsingError> {
        let data_directory = Self::get_directory(files_path)?;
        let files_paths = Self::get_files_paths(data_directory)?;
        match Self::get_sites(files_paths) {
            Ok(sites) => self.sites = Some(sites),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    /**
     * Returns the directory from the provided path. If the directory does not exist, it returns an error.
     */
    fn get_directory(files_path: &str) -> Result<ReadDir, ParsingError> {
        match read_dir(files_path) {
            Ok(directory) => Ok(directory),
            Err(_) => Err(ParsingError::ErrorCouldNotOpenDataDirectory),
        }
    }

    /**
     * Returns the paths of the files with json extension from the provided directory.
     * If no files are found, it returns an error.
     */
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
        if dir_entries.is_empty() {
            return Err(ParsingError::ErrorWhileBrowsingTheDirectoryEntries(
                "No files found".to_string(),
            ));
        }
        Ok(dir_entries)
    }

    /**
     * Returns the sites from the provided files paths using the provided amount of threads.
     * If there was an error while building the threadpool, it returns an error.
     */
    fn get_sites(files_paths: Vec<PathBuf>) -> Result<Vec<Site>, ParsingError> {
        let sites: Vec<Site> = files_paths
            .par_iter()
            .filter_map(|path| {
                if let Ok(file) = File::open(path) {
                    let file_name = path.file_name().map(|name| name.to_string_lossy().into_owned());
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
                                    eprintln!("{}", e);
                                    None
                                }
                            }
                        })
                        .fold(
                            || Site::new(file_name.clone()),
                            |mut site, question| {
                                site.add_question(question);
                                site
                            },
                        )
                        .reduce(
                            || Site::new(file_name.clone()),
                            |mut site, other| {
                                site.merge(other);
                                site
                            },
                        );
                    Some(site)
                } else {
                    None
                }
            })
            .collect();

        Ok(sites)
    }

    fn get_all_tags(&self) -> TagsCollection {
        match &self.sites {
            Some(sites) => sites
                .par_iter()
                .fold(
                    TagsCollection::new,
                    |mut tags, site| {
                        tags.merge_ref(site.get_tags());
                        tags
                    },
                )
                .reduce(
                    TagsCollection::new,
                    |mut tags, other| {
                        tags.merge(other);
                        tags
                    },
                ),
            None => TagsCollection::new(),
        }
    }

    pub fn generate_sites_jason(&self) -> Value {
        let mut sites_data = Value::Object(serde_json::Map::new());
        if let Some(sites) = &self.sites {
            for (index, site) in sites.iter().enumerate() {
                let name = match site.get_name() {
                    Some(name) => name,
                    None => index.to_string(),
                };
                sites_data[name] = site.generate_json();
            }
        }
        sites_data
    }

    pub fn get_chatty_sites(&self, number_of_sites: u8) -> Value {
        let ordered_sites = match &self.sites {
            Some(sites) => {
                let mut chatty_sites: Vec<&Site> = sites.iter().collect();
                chatty_sites.sort_by(|a, b| {
                    let ratio_a = a.calculate_words_questions_ratio();
                    let ratio_b = b.calculate_words_questions_ratio();
                    ratio_b
                        .partial_cmp(&ratio_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                chatty_sites
                    .into_iter()
                    .take(number_of_sites as usize)
                    .map(|site| {
                        Value::String(site.get_name().unwrap_or("Archivo sin nombre".to_string()))
                    })
                    .collect()
            }
            None => Vec::new(),
        };

        Value::Array(ordered_sites)
    }

    pub fn generate_totals_json(&self, tags: TagsCollection) -> Value {
        let mut data = Value::Object(serde_json::Map::new());
        data["chatty_sites"] = self.get_chatty_sites(10);
        data["chatty_tags"] = tags.generate_chatty_tags_json(10);
        data
    }

    pub fn generate_json_information(&self, padron: &str) -> Value {
        let mut data = Value::Object(serde_json::Map::new());
        data["padron"] = Value::String(padron.to_string());
        data["sites"] = self.generate_sites_jason();

        let tags = self.get_all_tags();

        data["tags"] = tags.generate_json();
        data["totals"] = self.generate_totals_json(tags);
        data
    }
}

impl Default for SitesCollection {
    fn default() -> Self {
        Self::new()
    }
}