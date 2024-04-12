/**
 * Represents the information of a question.
 */
#[derive(Debug)]
pub struct QuestionInformation {
    pub words: u16,
    pub tags: Vec<String>,
}

impl QuestionInformation {
    /**
     * Creates a new question information with the given word count and tags.
     */
    pub fn new(words: u16, tags: Vec<String>) -> Self {
        QuestionInformation { words, tags }
    }
}
