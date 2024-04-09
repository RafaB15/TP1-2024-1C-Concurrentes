use serde_json::Value;

use super::{
    question_information::QuestionInformation, tags_collection::TagsCollection
};

#[derive(Debug)]
pub struct Site {
    name: Option<String>,
    question_count: u32,
    word_count: u32,
    tags: TagsCollection,
}

impl Site {
    pub fn new(name: Option<String>) -> Self {
        Site {
            name,
            question_count: 0,
            word_count: 0,
            tags: TagsCollection::new(),
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.question_count += other.question_count;
        self.word_count += other.word_count;
        self.tags.merge(other.tags);
    }

    pub fn add_question(&mut self, question: QuestionInformation) {
        self.question_count += 1;
        self.word_count += question.words as u32;
        self.tags.add_tags(question.tags, question.words as u32);
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn generate_json(&self) -> Value {
        let mut site_data = Value::Object(serde_json::Map::new());
        site_data["questions"] = Value::from(self.question_count);
        site_data["words"] = Value::from(self.word_count);
        site_data["tags"] = self.tags.generate_json();
        site_data["chatty_tags"] = self.tags.generate_chatty_tags_json(10);
        site_data
    }

    pub fn get_tags(&self) -> &TagsCollection{
        &self.tags
    }

    pub fn print_info(&self) {
        println!(
            "I have {} questions and {} words",
            self.question_count, self.word_count
        );
    }
}
