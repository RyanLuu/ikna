use crate::note::Note;

#[derive(PartialEq, Debug)]
pub struct Card {
    question: String,
    answer: String,
}

pub struct CardType {
    name: String,
    question_format: String,
    answer_format: String,
}

impl Card {
    pub fn new(question: String, answer: String) -> Self {
        Self { question, answer }
    }
    pub fn from(card_type: &CardType, field_keys: &Vec<String>, note: &Note) -> Self {
        let mut new_card = Card::new(card_type.question_format.clone(), card_type.answer_format.clone());
        for (key, value) in field_keys.iter().zip(&note.field_values) {
            let search_string = format!("{{{{{}}}}}", key);
            new_card.question = new_card.question.replace(search_string.as_str(), value.as_str());
            new_card.answer = new_card.answer.replace(search_string.as_str(), value.as_str());
        }
        new_card
    }
}

impl CardType {
    pub fn new(name: String, question_format: String, answer_format: String) -> Self {
        Self {
            name,
            question_format,
            answer_format,
        }
    }
}
