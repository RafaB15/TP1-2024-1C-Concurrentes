#[derive(Debug)]
pub struct QuestionInformation {
    pub words: u16,
    pub tags: Vec<String>,
}

impl QuestionInformation {
    pub fn new(words: u16, tags: Vec<String>) -> Self {
        QuestionInformation { words, tags }
    }

    pub fn print_info(&self) {
        println!("Tengo {} palabras y {} tags", self.words, self.tags.len());
    }
}
