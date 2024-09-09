use serde_json::Value;

use super::tag_information::TagInformation;

use std::collections::HashMap;

/// Represents a collection of tags with their information.
#[derive(Debug)]
pub struct TagsCollection {
    tags: HashMap<String, TagInformation>,
}

impl TagsCollection {
    /// Creates a new tags collection.
    ///
    /// # Returns
    ///
    /// A new tags collection instance.
    pub fn new() -> Self {
        TagsCollection {
            tags: HashMap::new(),
        }
    }

    /// Merges the information of the tags collection with the information of another tags collection.
    /// It takes ownership of the other tags collection, making it unusable after the merge.
    ///
    /// # Arguments
    ///
    /// * `other` - The other tags collection to merge with
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

    /// Merges the information of the tags collection with the information of another tags collection.
    /// It does not take ownership of the other tags collection, so it can be used after the merge.
    ///
    /// # Arguments
    ///
    /// * `other` - The other tags collection to merge with
    pub fn merge_ref(&mut self, other: &Self) {
        for (tag, other_info) in &other.tags {
            match self.tags.get_mut(tag) {
                Some(tag_info) => {
                    tag_info.merge(*other_info);
                }
                None => {
                    self.tags.insert(tag.clone(), *other_info);
                }
            }
        }
    }

    /// Adds tags to the collection with the given word count.
    ///
    /// # Arguments
    ///
    /// * `tags` - The tags to add.
    /// * `words` - The word count of the tags.
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

    /// Generates a JSON with the tags collection information.
    ///
    /// # Returns
    ///
    /// A JSON with the tags collection information.
    pub fn generate_json(&self) -> Value {
        let mut tags_data = Value::Object(serde_json::Map::new());
        for (tag, tag_information) in &self.tags {
            tags_data[tag] = tag_information.generate_json();
        }
        tags_data
    }

    /// Generates a JSON with the most chatty tags.
    ///
    /// # Arguments
    ///
    /// * `number_of_tags` - The number of tags to include in the JSON.
    ///
    /// # Returns
    ///
    /// A JSON with the most chatty tags.
    pub fn generate_chatty_tags_json(&self, number_of_tags: u8) -> Value {
        let mut tags: Vec<(&String, &TagInformation)> = self.tags.iter().collect();
        tags.sort_by(|a, b| {
            let ratio_a = a.1.calculate_words_questions_ratio();
            let ratio_b = b.1.calculate_words_questions_ratio();
            ratio_b
                .partial_cmp(&ratio_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let top_tags = tags
            .into_iter()
            .take(number_of_tags as usize)
            .map(|(tag, _)| Value::String(tag.clone()))
            .collect();

        Value::Array(top_tags)
    }
}

impl Default for TagsCollection {
    /// Creates a new tags collection with the default values.
    ///
    /// # Returns
    ///
    /// A new tags collection instance.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tag_collection_is_empty() {
        let collection = TagsCollection::new();
        assert!(collection.tags.is_empty());
    }

    #[test]
    fn add_tags_to_collection() {
        let mut collection = TagsCollection::new();
        collection.add_tags(vec!["tag1".to_string(), "tag2".to_string()], 10);
        assert_eq!(collection.tags.len(), 2);
        assert_eq!(collection.tags.get("tag1").unwrap().word_count, 10);
        assert_eq!(collection.tags.get("tag2").unwrap().word_count, 10);
    }

    #[test]
    fn merge_collections() {
        let mut collection1 = TagsCollection::new();
        collection1.add_tags(vec!["tag1".to_string(), "tag2".to_string()], 10);

        let mut collection2 = TagsCollection::new();
        collection2.add_tags(vec!["tag1".to_string(), "tag3".to_string()], 20);

        collection1.merge(collection2);

        assert_eq!(collection1.tags.len(), 3);
        assert_eq!(collection1.tags.get("tag2").unwrap().word_count, 10);
        assert_eq!(collection1.tags.get("tag3").unwrap().word_count, 20);
        assert_eq!(collection1.tags.get("tag1").unwrap().word_count, 30);
    }
}
