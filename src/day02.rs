use std::collections::HashSet;
use std::vec;

use regex::Regex;

use crate::input::daily_input;

type Color = String;

struct Grab(Color, u32);

impl Grab {
    fn new(color: &str, count: u32) -> Grab {
        Grab(color.to_owned(), count)
    }
}

struct Hand {
    cubes: Vec<Grab>
}

impl Hand {
    fn empty() -> Hand {
        Hand::new(Vec::new())
    }

    fn new(picks: Vec<Grab>) -> Hand {
        Hand { cubes: picks }
    }

    fn parse(hand: &str) -> Hand {
        let re = Regex::new(r"(?<count>\d+) (?<color>(green|blue|red))").unwrap();

        let grabs = re.captures_iter(hand).map(|groups| {
            Grab(groups["color"].to_owned(), groups["count"].parse().unwrap())
        }).collect();

        Hand::new(grabs)
    }

    fn colors(self: &Self) -> HashSet<&Color> {
        HashSet::from_iter(self.cubes.iter().map(|Grab(color, _)| color))
    }

    fn count(self: &Self, color: &Color) -> u32 {
        self.cubes.iter().filter(|Grab(c, _)| c == color).map(|Grab(_, count)| count).sum()
    }

    fn power(self: &Self) -> u32 {
        self.colors().iter().fold(1, |power, color| power * self.count(color))
    }

    fn best_of_both(self: &Self, other: &Hand) -> Hand {
        let best_of = other.colors().union(&self.colors())
            .into_iter()
            .map(|&color| Grab(color.to_owned(), self.count(color).max(other.count(color))))
            .collect();

        Hand::new(best_of)
    }

    fn is_contained_in(self: &Self, other: &Hand) -> bool {
        self.cubes.iter().all(|Grab(color, count)| *count <= other.count(color))
    }
}

struct Game {
    id: u32,
    grabs: Vec<Hand>
}

impl Game {
    fn parse(game: &str) -> Game {
        let re = Regex::new(r"Game (?<id>\d+):").unwrap();
        let id = re.captures(game).unwrap()["id"].parse().unwrap();
        Game { id, grabs: game.split(";").map(Hand::parse).collect() }
    }

    fn is_possible_with_hand(self: &Self, hand: &Hand) -> bool {
        self.hand_required_to_play().is_contained_in(hand)
    }

    fn hand_required_to_play(self: &Self) -> Hand {
        self.grabs.iter().fold(Hand::empty(), |grab, other| grab.best_of_both(other))
    }
}

fn sum_possible_games(lines: Vec<String>) -> u32 {
    let available_cubes = Hand::new(vec![Grab::new("green", 13), Grab::new("red", 12), Grab::new("blue", 14)]);
    lines.iter().map(|game| Game::parse(game)).filter(|game| game.is_possible_with_hand(&available_cubes)).map(|game| game.id).sum()
}

fn sum_power_of_minimal_sets(lines: Vec<String>) -> u32 {
    lines.iter().map(|game| Game::parse(game)).map(|game| game.hand_required_to_play().power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let sum = sum_possible_games(daily_input(2));
        assert_eq!(sum, 2776);
    }

    #[test]
    fn part_two() {
        let sum = sum_power_of_minimal_sets(daily_input(2));
        assert_eq!(sum, 68638);
    }
}
