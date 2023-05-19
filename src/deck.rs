use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::card::Card;
use crate::note::{Note, NoteType};

pub struct Deck {
    name: String,
    notes: Vec<Rc<RefCell<Note>>>,
    options: DeckOptions,
}

pub struct DeckOptions {}

pub struct NoteIter<'a> {
    deck: &'a Deck,
    note: usize,
}

pub struct CardIter<'a> {
    deck: &'a Deck,
    note: usize,
    card: usize,
}

impl Deck {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notes: Vec::new(),
            options: DeckOptions::default(),
        }
    }

    pub fn add_note(&mut self, note_type: &mut NoteType, field_values: Vec<String>) -> &mut Rc<RefCell<Note>> {
        let new_note = Rc::new(RefCell::new(Note::new(field_values)));
        note_type.add_note(Rc::downgrade(&new_note));
        self.notes.push(new_note);
        self.notes.last_mut().unwrap()
    }

    pub fn note_iter(&self) -> NoteIter<'_> {
        NoteIter::new(self)
    }

    pub fn card_iter(&self) -> CardIter<'_> {
        CardIter::new(self)
    }
}

impl Default for DeckOptions {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> NoteIter<'a> {
    pub fn new(deck: &'a Deck) -> Self {
        Self { deck, note: 0 }
    }
}

impl<'a> CardIter<'a> {
    pub fn new(deck: &'a Deck) -> Self {
        Self { deck, note: 0, card: 0 }
    }
}

impl<'a> Iterator for NoteIter<'a> {
    type Item = &'a Rc<RefCell<Note>>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let note = self.deck.notes.get(self.note);
        self.note += 1;
        note
    }
}

impl<'a> ExactSizeIterator for NoteIter<'a> {
    fn len(&self) -> usize {
        return self.deck.notes.len();
    }
}

impl<'a> Iterator for CardIter<'a> {
    type Item = Ref<'a, Card>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let note = self.deck.notes.get(self.note);
        if let Some(note) = note {
            if self.card < note.borrow().cards.len() {
                let card = Ref::map(note.borrow(), |n| &n.cards[self.card]);
                self.card += 1;
                return Some(card);
            } else {
                self.note += 1;
                self.card = 0;
                return self.next();
            }
        }
        None
    }
}

impl<'a> ExactSizeIterator for CardIter<'a> {
    fn len(&self) -> usize {
        return self.deck.notes.iter().map(|note| note.borrow().cards.len()).sum();
    }
}
