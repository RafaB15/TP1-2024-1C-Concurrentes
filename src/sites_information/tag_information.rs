use serde_json::Value;

/**
 * Represents the information of a tag.
 */
#[derive(Debug, Clone, Copy)]
pub struct TagInformation {
    question_count: u32,
    word_count: u32,
}

impl TagInformation {
    /**
     * Creates a new tag information with the given word count.
     */
    pub fn new(word_count: u32) -> Self {
        TagInformation {
            question_count: 1,
            word_count,
        }
    }

    /**
     * Adds an appearance of the tag with the given word count.
     */
    pub fn add_appearance(&mut self, word_count: u32) {
        self.word_count += word_count;
        self.question_count += 1;
    }

    /**
     * Merges the information of the tag with the information of another tag.
     */
    pub fn merge(&mut self, other: Self) {
        self.question_count += other.question_count;
        self.word_count += other.word_count;
    }

    /**
     * Returns the word count of the tag.
     */
    pub fn calculate_words_questions_ratio(&self) -> f64 {
        self.word_count as f64 / self.question_count as f64
    }

    /**
     * Generates a json with the tag information.
     */
    pub fn generate_json(&self) -> Value {
        let mut tag_data = Value::Object(serde_json::Map::new());
        tag_data["questions"] = Value::from(self.question_count);
        tag_data["words"] = Value::from(self.word_count);
        tag_data
    }
}
