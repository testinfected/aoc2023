use regex::Regex;

#[derive(PartialEq)]
struct Card {
    id: u32,
    winning_numbers: Vec<String>,
    own_numbers: Vec<String>,
    is_copy: bool,
}

impl Card {
    fn parse<T: AsRef<str>>(card: T) -> Card {
        let regex = Regex::new(r"Card\s+(?<id>\d+): (?<winners>[\d\s]+) \| (?<own>[\d ]+)").unwrap();
        let (_, [id, winners, own]) = regex.captures(card.as_ref()).unwrap().extract();
        let winning_numbers = winners.split_whitespace().map(|n| n.to_string()).collect();
        let own_numbers = own.split_whitespace().map(|n| n.to_string()).collect();
        Card { id: id.parse().unwrap(), winning_numbers, own_numbers, is_copy: false }
    }

    fn winners_count(&self) -> u32 {
        self.own_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count() as u32
    }

    fn score(&self) -> u32 {
        let winners_count = self.winners_count();
        if winners_count == 0 { 0 } else { 2_u32.pow(winners_count - 1) }
    }

    fn copy(&self) -> Card {
        Card {
            id: self.id,
            own_numbers: self.own_numbers.clone(),
            winning_numbers: self.winning_numbers.clone(),
            is_copy: true,
        }
    }

    fn is_original(&self) -> bool {
        self.is_copy == false
    }
}

struct CardDeck {
    cards: Vec<Card>,
}

impl CardDeck {
    fn empty() -> CardDeck {
        CardDeck::new(Vec::new())
    }

    fn new(cards: Vec<Card>) -> CardDeck {
        CardDeck { cards: Vec::from(cards) }
    }

    fn count(&self) -> usize {
        self.cards.len()
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn claim_copy(&self, id: u32) -> Option<Card> {
        self.cards.iter().find(|c| c.id == id).map(|card| card.copy())
    }

    fn put(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn total_score(&self) -> u32 {
        self.cards.iter().map(|c| c.score()).sum()
    }
}

fn parse_cards<T: AsRef<str>>(cards: Vec<T>) -> Vec<Card> {
    cards.into_iter().map(|card| Card::parse(card.as_ref())).collect()
}


struct GameRules;

impl GameRules {
    fn claim_prizes(mut deck: CardDeck) -> (CardDeck, CardDeck) {
        let mut originals = CardDeck::empty();
        let mut prizes = CardDeck::empty();

        while let Some(card) = deck.draw() {
            for n in 1..=card.winners_count() {
                deck.put(originals.claim_copy(card.id + n).unwrap())
            }
            match card.is_original() {
                true => originals.put(card),
                false => prizes.put(card),
            }
        }
        (originals, prizes)
    }
}


#[cfg(test)]
mod test {
    use crate::input::{daily_example, daily_input};

    use super::*;

    #[test]
    fn knows_card_winning_numbers() {
        let cards = parse_cards(daily_example(4));
        let first = cards.first().unwrap();
        assert_eq!(first.winning_numbers, vec!["41", "48", "83", "86", "17"])
    }

    #[test]
    fn knows_card_own_numbers() {
        let cards = parse_cards(daily_example(4));
        let first = cards.first().unwrap();
        assert_eq!(first.own_numbers, vec!["83", "86", "6", "31", "17", "9", "48", "53"])
    }

    #[test]
    fn knows_card_winners() {
        let winning_counts: Vec<u32> = parse_cards(daily_example(4))
            .iter()
            .map(Card::winners_count)
            .collect();
        assert_eq!(winning_counts, vec![4, 2, 2, 1, 0, 0])
    }

    #[test]
    fn computes_card_score() {
        let cards = parse_cards(daily_example(4));
        let card_scores: Vec<u32> = cards.iter().map(|card| card.score()).collect();
        assert_eq!(card_scores, vec![8, 2, 2, 1, 0, 0])
    }

    #[test]
    fn solves_part_one() {
        let deck = CardDeck::new(parse_cards(daily_input(4)));
        assert_eq!(deck.total_score(), 27845)
    }

    #[test]
    fn gifts_card_copies() {
        let deck = CardDeck::new(parse_cards(daily_example(4)));
        let (originals, prizes) = GameRules::claim_prizes(deck);
        assert_eq!(originals.count(), 6);
        assert_eq!(prizes.count(), 24);
    }

    #[test]
    fn solves_part_two() {
        let deck = CardDeck::new(parse_cards(daily_input(4)));
        let (originals, prizes) = GameRules::claim_prizes(deck);
        assert_eq!(originals.count() + prizes.count(), 9496801);
    }
}