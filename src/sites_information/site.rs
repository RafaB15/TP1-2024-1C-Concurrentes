use super::{
    question_information::QuestionInformation,
    tags_collection::TagsCollection
};

#[derive(Debug)]
pub struct Site {
    question_count: u32,
    word_count: u32,
    tags: TagsCollection,
}

impl Site {
    pub fn new() -> Self {
        Site {
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

    pub fn get_question_count(&self) -> u32{
        self.question_count
    }

    pub fn get_word_count(&self) -> u32 {
        self.word_count
    }

    pub fn print_info(&self) {
        println!(
            "I have {} questions and {} words",
            self.question_count, self.word_count
        );
    }
}
