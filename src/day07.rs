use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::day07::Card::*;
use crate::day07::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Copy, Clone, Hash)]
enum Card {
    A = 15,
    K = 14,
    Q = 13,
    J = 12,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
    JOKER = 1,
}

impl Card {
    fn variants() -> impl Iterator<Item=Card> {
        [A, K, Q, J, T, _9, _8, _7, _6, _5, _4, _3, _2, JOKER].iter().cloned()
    }

    fn symbols() -> impl Iterator<Item=char> {
        ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '*'].iter().copied()
    }

    fn lookup_table() -> HashMap<char, Card> {
        HashMap::from_iter(Self::symbols().zip(Self::variants()))
    }

    fn lookup(symbol: char) -> Card {
        Self::lookup_table().get(&symbol).unwrap().clone()
    }
}

#[derive(PartialEq, Debug, Ord, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn with_jokers(self, count: u32) -> HandType {
        if count == 0 { return self }
        match (&self, count) {
            (FiveOfAKind, _) => FiveOfAKind,
            (FourOfAKind, _) => FiveOfAKind,
            (FullHouse, _) => FiveOfAKind,
            (ThreeOfAKind, _) => FourOfAKind,
            (TwoPair, 2) => FourOfAKind,
            (TwoPair, 1) => FullHouse,
            (OnePair, _) => ThreeOfAKind,
            (HighCard, _) => OnePair,
            _ => self
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn from_str(hand: &str) -> Self {
        let cards: Vec<Card> = hand.chars().map(Card::lookup).collect();
        Hand { cards: cards.try_into().unwrap() }
    }

    fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    fn organized_cards(&self) -> HashMap<&Card, usize> {
        self.cards.iter().counts()
    }

    fn evaluate(&self) -> HandType {
        let cards = self.organized_cards();
        let counts: Vec<usize> = cards.values().sorted().rev().copied().collect();
        let &jokers = cards.get(&JOKER).unwrap_or(&0);

        match counts.as_slice() {
            [5, ..] => FiveOfAKind,
            [4, ..] => FourOfAKind,
            [3, 2, ..] => FullHouse,
            [3, ..] => ThreeOfAKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => OnePair,
            [..] => HighCard,
        }.with_jokers(jokers as u32)
    }

    fn cmp_by_type(&self, other: &Hand) -> Ordering {
        self.evaluate().cmp(&other.evaluate())
    }

    fn cmp_by_card(&self, other: &Hand) -> Ordering {
        self.cards.iter().zip_eq(other.cards.iter())
            .find(|&(lhs, rhs)| lhs != rhs)
            .map(|(lhs, rhs)| lhs.cmp(rhs))
            .unwrap_or(Ordering::Equal)
    }

    fn bid(self, amount: u32) -> Bid {
        Bid { hand: self, amount }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_by_type(other).then(self.cmp_by_card(&other))
    }
}

#[derive(PartialEq, Debug)]
struct Bid {
    hand: Hand,
    amount: u32,
}

impl Bid {
    fn from_str<T: AsRef<str>>(bid: T) -> Bid {
        let numbers = bid.as_ref().split_whitespace();
        let &[hand, amount] = numbers.collect::<Vec<&str>>().as_slice().try_into().unwrap();
        Bid { hand: Hand::from_str(hand), amount: amount.parse().unwrap() }
    }
}

fn parse_bids(bids: Vec<String>) -> Vec<Bid> {
    bids.into_iter().map(Bid::from_str).collect()
}

fn parse_bids_using_jokers(bids: Vec<String>) -> Vec<Bid> {
    bids.iter()
        .map(|bid| bid.replace("J", "*"))
        .into_iter()
        .map(Bid::from_str)
        .collect()
}

fn total_winnings(bids: Vec<Bid>) -> u32 {
    bids.iter()
        .sorted_by_key(|b| &b.hand)
        .enumerate()
        .map(|(index, bid)| (index + 1) as u32 * bid.amount)
        .sum()
}

mod test {
    use crate::day07::HandType::{FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};
    use crate::input::{daily_example, daily_input};

    use super::*;

    #[test]
    fn parses_hand() {
        assert_eq!(Hand::from_str("KT82Q").cards(), &[K, T, _8, _2, Q])
    }

    #[test]
    fn knows_each_type_of_hand() {
        assert_eq!(Hand::from_str("AAAAA").evaluate(), FiveOfAKind);
        assert_eq!(Hand::from_str("AA8AA").evaluate(), FourOfAKind);
        assert_eq!(Hand::from_str("23332").evaluate(), FullHouse);
        assert_eq!(Hand::from_str("TTT98").evaluate(), ThreeOfAKind);
        assert_eq!(Hand::from_str("23432").evaluate(), TwoPair);
        assert_eq!(Hand::from_str("A23A4").evaluate(), OnePair);
        assert_eq!(Hand::from_str("23456").evaluate(), HighCard);
    }

    #[test]
    fn knows_which_hand_has_stronger_first_card() {
        assert!(Hand::from_str("33332") > Hand::from_str("2AAAA"));
        assert!(Hand::from_str("77888") > Hand::from_str("77788"));
        assert_eq!(Hand::from_str("23456"), Hand::from_str("23456"));
    }

    #[test]
    fn knows_hands_relative_strengths() {
        assert!(Hand::from_str("55555") > Hand::from_str("KAAAA"));
        assert!(Hand::from_str("78888") > Hand::from_str("88877"));
        assert!(Hand::from_str("77888") > Hand::from_str("TTA66"));
        assert!(Hand::from_str("JJ7TT") > Hand::from_str("KKAQJ"));
        assert!(Hand::from_str("JJ762") > Hand::from_str("K89QJ"));
        assert!(Hand::from_str("J9762") > Hand::from_str("J975A"));
    }

    #[test]
    fn parses_bids() {
        let bids = parse_bids(daily_example(7));
        assert_eq!(bids, vec![
            Hand::from_str("32T3K").bid(765),
            Hand::from_str("T55J5").bid(684),
            Hand::from_str("KK677").bid(28),
            Hand::from_str("KTJJT").bid(220),
            Hand::from_str("QQQJA").bid(483),
        ])
    }

    #[test]
    fn calculates_total_winnings() {
        assert_eq!(total_winnings(parse_bids(daily_example(7))), 6440)
    }

    #[test]
    fn solves_part_one() {
        assert_eq!(total_winnings(parse_bids(daily_input(7))), 241344943)
    }

    #[test]
    fn knows_hand_types_containing_jokers() {
        assert_eq!(Hand::from_str("32T3K").evaluate(), OnePair);
        assert_eq!(Hand::from_str("KK677").evaluate(), TwoPair);
        assert_eq!(Hand::from_str("T55*5").evaluate(), FourOfAKind);
        assert_eq!(Hand::from_str("KT**T").evaluate(), FourOfAKind);
        assert_eq!(Hand::from_str("QQ**A").evaluate(), FourOfAKind);
    }

    #[test]
    fn calculates_total_winnings_using_jokers() {
        assert_eq!(total_winnings(parse_bids_using_jokers(daily_example(7))), 5905)
    }

    #[test]
    fn solves_part_two() {
        assert_eq!(total_winnings(parse_bids_using_jokers(daily_input(7))), 243101568)
    }
}