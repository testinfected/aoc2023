use regex::Regex;

use crate::input::daily_input;

#[derive(PartialEq, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    const AROUND: [(isize, isize); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];

    fn vicinity(self: &Self) -> Vec<Pos> {
        Pos::AROUND.iter()
            .map(|(dx, dy)| Pos { x: self.x + dx, y: self.y + dy })
            .collect()
    }
}

struct Region {
    locations: Vec<Pos>,
    visual: String,
}

impl Region {
    fn is_in_vicinity_of(self: &Self, pos: &Pos) -> bool {
        pos.vicinity().iter().any(|p| self.locations.contains(p))
    }

    fn is_adjacent_to(self: &Self, other: &Region) -> bool {
        other.locations.iter().any(|p| self.is_in_vicinity_of(p))
    }

    fn number(self: &Self) -> Option<u32> {
        self.visual.parse().ok()
    }

    fn to_star_symbol(self: Self) -> Option<Region> {
        if self.visual == "*" { Some(self) } else { None }
    }

    fn ratio(self: &Self, neighbors: Vec<&Region>) -> Option<u32> {
        if neighbors.len() == 2 {
            Some(neighbors.iter().filter_map(|neighbor| neighbor.number()).product())
        } else {
            None
        }
    }
}

struct Schematics {
    width: usize,
    visual: String,
}

struct Gear(Region, u32);

impl Schematics {
    fn parse(lines: Vec<String>) -> Schematics {
        let visual = lines.join("");
        let width = visual.len() / lines.len();
        Schematics { width, visual }
    }

    fn regions_matching(self: &Self, re: Regex) -> Vec<Region> {
        re.captures_iter(&self.visual)
            .filter_map(|captures| captures.name("region"))
            .map(|m| Region {
                locations: (m.start()..m.end()).map(|offset| self.to_pos(offset)).collect(),
                visual: m.as_str().to_owned(),
            })
            .collect()
    }

    fn numbers(self: &Self) -> Vec<Region> {
        self.regions_matching(Regex::new(r"(?<region>\d+)").unwrap())
    }

    fn symbols(self: &Self) -> Vec<Region> {
        self.regions_matching(Regex::new(r"(?<region>[^.\d])").unwrap())
    }

    fn parts(self: &Self) -> Vec<Region> {
        let symbols = self.symbols();
        self.numbers()
            .into_iter()
            .filter(|number| symbols.iter().any(|symbol| number.is_adjacent_to(&symbol)))
            .collect()
    }

    fn part_numbers(self: &Self) -> Vec<u32> {
        self.parts().iter().filter_map(|part| part.visual.parse().ok()).collect::<Vec<u32>>()
    }

    fn gears(self: &Self) -> Vec<Gear> {
        let parts = self.parts();
        self.symbols()
            .into_iter()
            .filter_map(|symbol| symbol.to_star_symbol())
            .filter_map(|star| {
                let ratio = star.ratio(parts.iter().filter(|part| part.is_adjacent_to(&star)).collect());
                ratio.map(|it| Gear(star, it))
            })
            .collect()
    }

    fn gear_ratios(self: &Self) -> Vec<u32> {
        self.gears().into_iter().map(|Gear(_, ratio)| ratio).collect()
    }

    fn to_pos(self: &Self, offset: usize) -> Pos {
        Pos {
            x: (offset % self.width) as isize,
            y: (offset / self.width) as isize,
        }
    }
}

fn sum_of_part_numbers(lines: Vec<String>) -> u32 {
    let schematics = Schematics::parse(lines);
    schematics.part_numbers().iter().sum()
}

fn sum_of_gear_ratios(lines: Vec<String>) -> u32 {
    let schematics = Schematics::parse(lines);
    schematics.gear_ratios().iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::input::daily_example;

    use super::*;

    #[test]
    fn finds_all_numbers() {
        let schematics = Schematics::parse(daily_example(3));
        let numbers: Vec<String> = schematics.numbers().into_iter().map(|n| n.visual).collect();
        assert_eq!(numbers, vec!["467", "114", "35", "633", "617", "58", "592", "755", "664", "598"])
    }

    #[test]
    fn finds_parts() {
        let schematics = Schematics::parse(daily_example(3));
        let parts: Vec<String> = schematics.parts().into_iter().map(|n| n.visual).collect();
        assert_eq!(parts, vec!["467", "35", "633", "617", "592", "755", "664", "598"])
    }

    #[test]
    fn computes_sum_of_part_numbers() {
        assert_eq!(sum_of_part_numbers(daily_example(3)), 4361)
    }

    #[test]
    fn solves_part_one() {
        let sum = sum_of_part_numbers(daily_input(3));
        assert_eq!(sum, 528819);
    }

    #[test]
    fn finds_gears() {
        let schematics = Schematics::parse(daily_example(3));
        let gears_locations: Vec<Pos> = schematics.gears().into_iter().flat_map(|Gear(region, _)| region.locations).collect();
        assert_eq!(gears_locations, vec![Pos{ x: 3, y: 1}, Pos{ x: 5, y: 8}])
    }

    #[test]
    fn sums_gear_ratios() {
        assert_eq!(sum_of_gear_ratios(daily_example(3)), 467835)
    }

    #[test]
    fn solves_part_two() {
        assert_eq!(sum_of_gear_ratios(daily_input(3)), 80403602)
    }
}
