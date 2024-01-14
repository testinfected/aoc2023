use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

use Component::{Fertilizer, Humidity, Light, Location, Seed, Soil, Temperature, Water};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
enum Component {
    Seed(isize),
    Soil(isize),
    Fertilizer(isize),
    Water(isize),
    Light(isize),
    Temperature(isize),
    Humidity(isize),
    Location(isize),
}

impl Component {
    fn parse(name: &str) -> ComponentKind {
        match name {
            "seed" => Seed,
            "soil" => Soil,
            "fertilizer" => Fertilizer,
            "water" => Water,
            "light" => Light,
            "temperature" => Temperature,
            "humidity" => Humidity,
            "location" => Location,
            _ => panic!()
        }
    }

    fn number(&self) -> isize {
        match self {
            &Seed(n) | &Soil(n) | &Fertilizer(n) | &Water(n) |
            &Light(n) | &Temperature(n) | &Humidity(n) | &Location(n) => { n }
        }
    }

    fn is_a(&self, kind: ComponentKind) -> bool {
        *self == kind(self.number())
    }
}

type ComponentKind = fn(isize) -> Component;

struct CorrelationRule {
    range: Range<isize>,
    offset: isize,
}

impl CorrelationRule {
    fn apply(&self, number: isize) -> Option<isize> {
        if self.range.contains(&number) { Some(number + self.offset) } else { None }
    }
}

struct CorrelationTable {
    source: ComponentKind,
    destination: ComponentKind,
    rules: Vec<CorrelationRule>,
}

impl CorrelationTable {
    fn parse(table: &str) -> CorrelationTable {
        let mut lines = table.lines();
        let (source, destination) = Self::parse_header(lines.next().unwrap());
        let rules = lines.map(Self::parse_rule).collect();
        CorrelationTable { source, destination, rules }
    }

    fn parse_header(header: &str) -> (ComponentKind, ComponentKind) {
        let re = Regex::new(r"(?<source>\w+)-to-(?<destination>\w+) map:").unwrap();
        let (_, [source, destination]) = re.captures(header).unwrap().extract();
        (Component::parse(source), Component::parse(destination))
    }

    fn parse_rule(rule: &str) -> CorrelationRule {
        let numbers: Vec<isize> = rule.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let &[to, from, range_length] = numbers.as_slice() else { panic!() };
        CorrelationRule { range: from..(from + range_length), offset: to - from }
    }

    fn lookup(&self, component: Component) -> Option<Component> {
        if !component.is_a(self.source) { return None; };
        let number = self.rules.iter()
            .find_map(|rule| rule.apply(component.number()))
            .unwrap_or(component.number());
        Some((self.destination)(number))
    }
}

struct SeedBag {
    seeds: Vec<Component>,
}

impl SeedBag {
    fn parse(bag: &str) -> Self {
        SeedBag { seeds: parse_seed_numbers(bag).iter().map(|&n| Seed(n)).collect() }
    }

    fn iter(self) -> impl Iterator<Item=Component> {
        self.seeds.into_iter()
    }
}

struct SeedField {
    range: Range<isize>,
}

impl SeedField {
    fn iter(&self) -> impl Iterator<Item=Component> {
        self.range.clone().map(Seed)
    }
}

struct SeedFarm {
    fields: Vec<SeedField>,
}

impl SeedFarm {
    fn iter(&self) -> impl Iterator<Item=Component> + '_ {
        self.fields.iter().flat_map(|field| field.iter())
    }

    fn parse(farm: &str) -> Self {
        SeedFarm::new(parse_seed_numbers(farm))
    }

    fn new(numbers: Vec<isize>) -> Self {
        let fields = numbers.as_slice().chunks_exact(2).map(|chunk| {
            let &[start, count] = chunk else { panic!() };
            SeedField { range: start..(start + count) }
        }).collect();

        SeedFarm { fields }
    }
}

struct Almanac {
    tables: Vec<CorrelationTable>,
}

impl Almanac {
    fn parse(instructions: &[String]) -> Self {
        let tables = instructions.join("\n")
            .split("\n\n").map(CorrelationTable::parse)
            .collect::<Vec<CorrelationTable>>();

        Almanac { tables }
    }

    fn correlate(&self, component: Component) -> Option<Component> {
        self.tables.iter().find_map(|table| table.lookup(component))
    }

    fn location_for(&self, component: Component) -> Option<Component> {
        self.tables.iter().fold(Some(component), |result, table| match result {
            Some(c) => {
                table.lookup(c)
            }
            None => None
        })
    }

    fn lowest_location_number_of(&self, seeds: impl Iterator<Item=Component> + Sized) -> Option<isize> {
        seeds.filter_map(|seed| self.location_for(seed))
            .map(|c| c.number())
            .min()
    }
}

fn parse_seed_numbers(spec: &str) -> Vec<isize> {
    let re = Regex::new(r"seeds:\s+(?<seeds>[\d\s]+)").unwrap();
    let (_, [seeds]) = re.captures(spec).unwrap().extract();
    seeds.split_whitespace()
        .filter_map(|n| isize::from_str(n).ok())
        .collect()
}

fn parse_instructions(instructions: Vec<String>) -> (SeedBag, Almanac) {
    (SeedBag::parse(&instructions[0]), Almanac::parse(&instructions[2..]))
}

fn parse_updated_instructions(instructions: Vec<String>) -> (SeedFarm, Almanac) {
    (SeedFarm::parse(&instructions[0]), Almanac::parse(&instructions[2..]))
}

mod test {
    use Component::{Seed, Soil};

    use crate::input::{daily_example, daily_input};

    use super::*;

    #[test]
    fn knows_seeds_to_be_planted() {
        let (bag, _) = parse_instructions(daily_example(5));
        assert_eq!(bag.seeds, vec![Seed(79), Seed(14), Seed(55), Seed(13)])
    }

    #[test]
    fn knows_correlation_between_components() {
        let (_, almanac) = parse_instructions(daily_example(5));
        assert_eq!(almanac.correlate(Seed(51)), Some(Soil(53)));
        assert_eq!(almanac.correlate(Seed(25)), Some(Soil(25)));
        assert_eq!(almanac.correlate(Soil(53)), Some(Fertilizer(38)));
    }

    #[test]
    fn correlate_seed_to_location() {
        let (_, almanac) = parse_instructions(daily_example(5));
        assert_eq!(almanac.location_for(Seed(79)), Some(Location(82)));
        assert_eq!(almanac.location_for(Seed(14)), Some(Location(43)));
        assert_eq!(almanac.location_for(Seed(55)), Some(Location(86)));
        assert_eq!(almanac.location_for(Seed(13)), Some(Location(35)));
    }

    #[test]
    fn finds_lowest_location_number() {
        let (seeds, almanac) = parse_instructions(daily_example(5));
        assert_eq!(almanac.lowest_location_number_of(seeds.iter()), Some(35))
    }

    #[test]
    fn solves_part_one() {
        let (seeds, almanac) = parse_instructions(daily_input(5));
        assert_eq!(almanac.lowest_location_number_of(seeds.iter()), Some(309796150))
    }

    #[test]
    fn finds_lowest_location_number_for_seed_ranges() {
        let (seeds, almanac) = parse_updated_instructions(daily_example(5));
        assert_eq!(almanac.lowest_location_number_of(seeds.iter()), Some(46))
    }

    #[test]
    fn solves_part_two() {
        let (seeds, almanac) = parse_updated_instructions(daily_input(5));
        assert_eq!(almanac.lowest_location_number_of(seeds.iter()), Some(50716416))
    }
}