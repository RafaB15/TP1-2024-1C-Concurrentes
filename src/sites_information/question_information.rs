#[derive(Debug)]
pub struct QuestionInformation {
    words: usize,
    tags: Vec<String>
}

impl QuestionInformation {
    pub fn new(words: usize, tags: Vec<String>) -> Self {
        QuestionInformation { words, tags }
    }

    pub fn print_info(&self) {
        println!("Tengo {} palabras y {} tags", self.words, self.tags.len());
    }
}