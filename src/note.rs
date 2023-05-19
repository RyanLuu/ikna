use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::card::{Card, CardType};

pub struct Note {
    pub field_values: Vec<String>,
    pub cards: Vec<Card>,
}

pub struct NoteType {
    field_keys: Vec<String>,
    card_types: Vec<CardType>,
    notes: Vec<Weak<RefCell<Note>>>,
}

impl Note {
    pub fn new(field_values: Vec<String>) -> Self {
        Self {
            field_values,
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card_type: &CardType, field_keys: &Vec<String>) -> &mut Card {
        self.cards.push(Card::from(card_type, field_keys, self));
        self.cards.last_mut().unwrap()
    }
}

impl NoteType {
    pub fn new(field_keys: Vec<String>) -> Self {
        Self {
            field_keys,
            card_types: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn add_card_type(&mut self, name: String, question_format: String, answer_format: String) -> &mut CardType {
        let new_card_type = CardType::new(name, question_format, answer_format);
        self.notes.retain_mut(|note| {
            if let Some(note) = note.upgrade() {
                note.borrow_mut().add_card(&new_card_type, &self.field_keys);
                true
            } else {
                false
            }
        });
        self.card_types.push(new_card_type);
        self.card_types.last_mut().unwrap()
    }

    pub fn add_note(&mut self, note: Weak<RefCell<Note>>) {
        self.notes.push(note);
    }
}
