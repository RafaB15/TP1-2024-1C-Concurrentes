use super::question_information::QuestionInformation;

#[derive(Debug)]
pub struct Site {
    questions: Vec<QuestionInformation>,
}

impl Site {
    pub fn new(questions: Vec<QuestionInformation>) -> Self {
        Site { questions }
    }

    pub fn print_info(&self) {
        for question in self.questions.iter() {
            question.print_info();
        }
    }
}