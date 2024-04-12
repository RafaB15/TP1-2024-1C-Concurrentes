use serde_json::Value;

use super::{question_information::QuestionInformation, tags_collection::TagsCollection};

/**
 * Represents a site with its information.
 */
#[derive(Debug)]
pub struct Site {
    name: Option<String>,
    question_count: u32,
    word_count: u32,
    tags: TagsCollection,
}

impl Site {
    /**
     * Creates a new site with the given name.
     */
    pub fn new(name: Option<String>) -> Self {
        Site {
            name,
            question_count: 0,
            word_count: 0,
            tags: TagsCollection::new(),
        }
    }

    /**
     * Merges the information of the site with the information of another site.
     * It takes ownership of the other site, making it unusable after the merge.
     */
    pub fn merge(&mut self, other: Self) {
        self.question_count += other.question_count;
        self.word_count += other.word_count;
        self.tags.merge(other.tags);
    }

    /**
     * Adds a question to the site.
     */
    pub fn add_question(&mut self, question: QuestionInformation) {
        self.question_count += 1;
        self.word_count += question.words as u32;
        self.tags.add_tags(question.tags, question.words as u32);
    }

    /**
     * Returns the name of the site.
     */
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    /**
     * Generates a json with the site information.
     */
    pub fn generate_json(&self) -> Value {
        let mut site_data = Value::Object(serde_json::Map::new());
        site_data["questions"] = Value::from(self.question_count);
        site_data["words"] = Value::from(self.word_count);
        site_data["tags"] = self.tags.generate_json();
        site_data["chatty_tags"] = self.tags.generate_chatty_tags_json(10);
        site_data
    }

    /**
     * Returns the tags of the questions in the site.
     */
    pub fn get_tags(&self) -> &TagsCollection {
        &self.tags
    }

    /**
     * Calculates the ratio between the amount of words and the amount of questions in the site.
     */
    pub fn calculate_words_questions_ratio(&self) -> f64 {
        self.word_count as f64 / self.question_count as f64
    }
}
