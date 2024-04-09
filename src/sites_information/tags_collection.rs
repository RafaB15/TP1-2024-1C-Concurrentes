use serde_json::Value;

use super::tag_information::TagInformation;

use std::collections::HashMap;

#[derive(Debug)]
pub struct TagsCollection {
    tags: HashMap<String, TagInformation>,
}

impl TagsCollection {
    pub fn new() -> Self {
        TagsCollection { tags: HashMap::new() }
    }

    pub fn merge(&mut self, other: Self) {
        for (tag, other_info) in other.tags {
            match self.tags.get_mut(&tag) {
                Some(tag_info) => {
                    tag_info.merge(other_info);
                }
                None => {
                    self.tags.insert(tag, other_info);
                }
            };
        }
    }

    pub fn add_tags(&mut self, tags: Vec<String>, words: u32) {
        for tag in tags {
            match self.tags.get_mut(&tag) {
                Some(tag_info) => {
                    tag_info.add_appearance(words);
                }
                None => {
                    self.tags.insert(tag, TagInformation::new(words));
                }
            }
        }
    }
    
    pub fn generate_json(&self) -> Value {
        let mut tags_data = Value::Object(serde_json::Map::new());
        for (tag, tag_information) in &self.tags {
            tags_data[tag] = tag_information.generate_json();
        }
        tags_data
    }

    pub fn generate_chatty_tags_json(&self, number_of_tags: u8) -> Value {
        let mut tags: Vec<(&String, &TagInformation)> = self.tags.iter().collect();
        tags.sort_by(|a, b| {
            let ratio_a = a.1.calculate_words_questions_ratio();
            let ratio_b = b.1.calculate_words_questions_ratio();
            ratio_b.partial_cmp(&ratio_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        let top_tags = tags.into_iter()
                                                                .take(number_of_tags as usize)
                                                                .map(|(tag, _)| Value::String(tag.clone()))
                                                                .collect();

        Value::Array(top_tags)                                                                
    }
}