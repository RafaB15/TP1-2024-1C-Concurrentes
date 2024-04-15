use serde::Deserialize;

use super::question_information::QuestionInformation;

/**
 * Represents a question with its texts and tags.
 */
#[derive(Debug, Deserialize)]
pub struct Question {
    texts: Vec<String>,
    tags: Vec<String>,
}

impl Question {
    /**
     * Converts the question into a QuestionInformation instance.
     */
    pub fn into_information(self) -> QuestionInformation {
        let word_count: usize = self.texts.iter().map(|text| text.len()).sum();
        QuestionInformation::new(word_count as u16, self.tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_information() {
        let question = Question {
            texts: vec!["Hello".to_string(), "World".to_string()],
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };
        let question_information = question.into_information();
        assert_eq!(question_information.words, 10);
        assert_eq!(
            question_information.tags,
            vec!["tag1".to_string(), "tag2".to_string()]
        );
    }
}
