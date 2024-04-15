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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let site = Site::new(Some("site".to_string()));
        assert_eq!(site.name, Some("site".to_string()));
        assert_eq!(site.question_count, 0);
        assert_eq!(site.word_count, 0);
    }

    #[test]
    fn test_merge() {
        let mut site1 = Site::new(Some("site1".to_string()));
        site1.question_count = 10;
        site1.word_count = 100;
        site1.tags.add_tags(vec!["tag1".to_string()], 10);

        let mut site2 = Site::new(Some("site2".to_string()));
        site2.question_count = 20;
        site2.word_count = 200;
        site2.tags.add_tags(vec!["tag2".to_string()], 20);

        site1.merge(site2);

        assert_eq!(site1.question_count, 30);
        assert_eq!(site1.word_count, 300);
    }

    #[test]
    fn test_add_question() {
        let mut site = Site::new(Some("site".to_string()));
        let question = QuestionInformation {
            words: 10,
            tags: vec!["tag1".to_string()],
        };
        site.add_question(question);

        assert_eq!(site.question_count, 1);
        assert_eq!(site.word_count, 10);
    }

    #[test]
    fn test_get_name() {
        let site = Site::new(Some("site".to_string()));
        assert_eq!(site.get_name(), Some("site".to_string()));
    }

    #[test]
    fn test_generate_json() {
        let mut site = Site::new(Some("site".to_string()));
        let question = QuestionInformation {
            words: 10,
            tags: vec!["tag1".to_string()],
        };
        site.add_question(question);

        let site_data = site.generate_json();
        assert_eq!(site_data["questions"], Value::from(1));
        assert_eq!(site_data["words"], Value::from(10));
        assert_eq!(site_data["tags"]["tag1"]["questions"], Value::from(1));
        assert_eq!(site_data["tags"]["tag1"]["words"], Value::from(10));
    }
}
