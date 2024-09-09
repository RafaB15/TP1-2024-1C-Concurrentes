/// Question information struct.
///
/// # Attributes
///
/// * `words` - The number of words in the question.
/// * `tags` - The tags of the question.
#[derive(Debug)]
pub struct QuestionInformation {
    pub words: u16,
    pub tags: Vec<String>,
}

impl QuestionInformation {
    /// Creates a new question information with the given word count and tags.
    ///
    /// # Arguments
    ///
    /// * `words` - The number of words in the question.
    /// * `tags` - The tags of the question.
    ///
    /// # Returns
    ///
    /// A new question information instance.
    pub fn new(words: u16, tags: Vec<String>) -> Self {
        QuestionInformation { words, tags }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let question_information =
            QuestionInformation::new(10, vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(question_information.words, 10);
        assert_eq!(
            question_information.tags,
            vec!["tag1".to_string(), "tag2".to_string()]
        );
    }
}
