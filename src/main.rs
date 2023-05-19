mod card;
mod deck;
mod note;

use crate::card::Card;
use crate::deck::Deck;
use crate::note::NoteType;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        const N: usize = 5;

        let mut note_type = NoteType::new(vec!["English".into(), "Chinese".into()]);
        let mut deck = Deck::new("My Deck".into());
        for i in 0..N {
            deck.add_note(&mut note_type, vec![format!("foo{}", i), format!("bar{}", i)]);
        }
        assert_eq!(deck.card_iter().len(), 0);
        note_type.add_card_type("A en-zh".into(), "A en: {{English}}".into(), "A zh: {{Chinese}}".into());
        note_type.add_card_type("B zh-en".into(), "B zh: {{Chinese}}".into(), "B en: {{English}}".into());
        let card_iter = deck.card_iter();
        assert_eq!(card_iter.len(), 2 * N);
        for (i, card) in card_iter.enumerate() {
            let expected = if i % 2 == 0 {
                Card::new(format!("A en: foo{}", i / 2), format!("A zh: bar{}", i / 2))
            } else {
                Card::new(format!("B zh: bar{}", i / 2), format!("B en: foo{}", i / 2))
            };
            assert_eq!(*card, expected);
        }
    }
}
