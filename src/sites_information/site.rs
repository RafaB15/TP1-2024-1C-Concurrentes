use super::{
    question_information::QuestionInformation,
    tag_information::TagInformation,
};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Site {
    question_count: u32,
    word_count: u32,
    tags: HashMap<String, TagInformation>,
}

impl Site {
    pub fn new() -> Self {
        Site {question_count: 0, word_count: 0, tags: HashMap::new() }
    }

    pub fn merge(&mut self, other: Self) {
        self.question_count += other.question_count;
        self.word_count += other.word_count;
        for (tag, other_info) in other.tags {
            match self.tags.get_mut(&tag) {
                Some(tag_info) => {
                    tag_info.merge(other_info);
                },
                None => {
                    self.tags.insert(tag, other_info);        
                },
            }
        }
    }

    pub fn add_question(&mut self, question: QuestionInformation) {
        self.question_count += 1;
        self.word_count += question.words as u32;
        
        for tag in question.tags {
            match self.tags.get_mut(&tag) {
                Some(tag_info) => {
                    tag_info.add_appearance(question.words as u32);
                },
                None => {
                    self.tags.insert(tag, TagInformation::new(question.words as u32));        
                },
            }
        }
    } 

    pub fn print_info(&self) {
        println!("I have {} questions and {} words", self.question_count, self.word_count);
    }
    
}