use serde::Deserialize;

use super::question_information::QuestionInformation;

#[derive(Debug, Deserialize)]
pub struct Question {
    texts: Vec<String>,
    tags: Vec<String>,
}

impl Question {
    pub fn into_information(self) -> QuestionInformation {
        let word_count: usize = self.texts.iter().map(|text| text.len()).sum();
        QuestionInformation::new(word_count as u16, self.tags)
    }
}
