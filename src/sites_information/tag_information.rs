#[derive(Debug)]
pub struct TagInformation {
    question_count: u32,
    word_count: u32,
}

impl TagInformation {
    pub fn new(word_count: u32) -> Self {
        TagInformation {question_count: 1, word_count}
    }

    pub fn add_appearance(&mut self, word_count: u32) {
        self.word_count += word_count;
        self.question_count += 1;
    }

    pub fn merge(&mut self, other: Self) {
        self.question_count += other.question_count;
        self.word_count += other.word_count;
    }
}
