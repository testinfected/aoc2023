use itertools::Itertools;
use num::integer;
use regex::Regex;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Node(String);

impl Node {
    fn new(label: &str) -> Node {
        Node(label.to_string())
    }

    fn is_start_node(&self) -> bool {
        self.0.ends_with("A")
    }

    fn is_end_node(&self) -> bool {
        self.0.ends_with("Z")
    }
}

#[derive(PartialEq, Debug)]
struct Connection {
    from: Node,
    left: Node,
    right: Node,
}

impl Connection {
    fn from_str(s: &str) -> Connection {
        let re = Regex::new(r"(?<from>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
        let (_, [from, left, right]) = re.captures(s).unwrap().extract();
        Connection { from: Node::new(from), left: Node::new(left), right: Node::new(right) }
    }

    fn navigate(&self, direction: Direction) -> &Node {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(PartialEq)]
struct Network {
    connections: Vec<Connection>,
}

impl Network {
    fn from_map(lines: &[String]) -> Network {
        let connections = lines.iter().map(|each| Connection::from_str(each)).collect();
        Network { connections }
    }

    fn starting_node() -> Node {
        Node::new("AAA")
    }

    fn take_step(&self, from_node: &Node, in_direction: Direction) -> Option<&Node> {
        self.connections.iter()
            .find(|c| *from_node == c.from)
            .map(|c| c.navigate(in_direction))
    }

    fn navigate_path_to_end<'a>(&'a self, start_node: Node, instructions: &'a Instructions) -> Path<'a> {
        Box::new(self.navigate(start_node, instructions).take_while_inclusive(|&node| !node.is_end_node()))
    }

    fn navigate_from_start_to_end<'a>(&'a self, instructions: &'a Instructions) -> Path<'a> {
        self.navigate_path_to_end(Self::starting_node(), instructions)
    }

    fn navigate_from_start_to_end_simultaneously<'a>(&'a self, instructions: &'a Instructions) -> Vec<Path<'a>> {
        self.start_nodes()
            .map(|start| self.navigate_path_to_end(start, instructions))
            .collect::<Vec<Path>>()
    }

    fn start_nodes(&self) -> impl Iterator<Item=Node> + '_ {
        self.connections.iter()
            .map(|c| c.from.clone())
            .filter(|node| node.is_start_node())
    }

    fn navigate<'a>(&'a self, from_node: Node, instructions: &'a Instructions) -> Path<'a> {
        Box::new(instructions.iter().cycle()
            .scan(from_node, |node, &direction| {
                self.take_step(node, direction).map(|next| {
                    node.clone_from(next);
                    next
                })
            })
        )
    }
}

type Path<'a> = Box<dyn Iterator<Item=&'a Node> + 'a>;

type Instructions = Vec<Direction>;

fn parse_instructions(input: &str) -> Instructions {
    input.chars().map(|c| Direction::from_char(c)).collect()
}

fn parse_input(input: Vec<String>) -> (Instructions, Network) {
    let instructions = parse_instructions(&input[0]);
    let network = Network::from_map(&input[2..]);
    (instructions, network)
}

fn total_steps(input: Vec<String>) -> usize {
    let (instructions, network) = parse_input(input);
    let path = network.navigate_from_start_to_end(&instructions);
    path.count()
}

fn total_steps_as_ghost(input: Vec<String>) -> usize {
    let (instructions, network) = parse_input(input);
    let paths = network.navigate_from_start_to_end_simultaneously(&instructions);
    paths.into_iter().map(|p| p.count()).reduce(|a, b| integer::lcm(a, b)).unwrap_or(0)
}

mod test {
    use crate::day08::Direction::{Left, Right};
    use crate::input::{daily_example, daily_input};

    use super::*;

    #[test]
    fn parses_instructions() {
        let (instructions, _) = parse_input(daily_example(8));

        assert_eq!(instructions, vec![Left, Left, Right])
    }

    #[test]
    fn parses_network_of_nodes() {
        let (_, network) = parse_input(daily_example(8));

        assert_eq!(network.take_step(&Node::new("AAA"), Left), Some(&Node::new("BBB")));
        assert_eq!(network.take_step(&Node::new("AAA"), Right), Some(&Node::new("BBB")));
        assert_eq!(network.take_step(&Node::new("BBB"), Left), Some(&Node::new("AAA")));
        assert_eq!(network.take_step(&Node::new("BBB"), Right), Some(&Node::new("ZZZ")));
        assert_eq!(network.take_step(&Node::new("ZZZ"), Left), Some(&Node::new("ZZZ")));
        assert_eq!(network.take_step(&Node::new("ZZZ"), Right), Some(&Node::new("ZZZ")));
    }

    #[test]
    fn counts_step_to_navigate_network() {
        assert_eq!(total_steps(daily_example(8)), 6);
    }

    #[test]
    fn solves_part_one() {
        assert_eq!(total_steps(daily_input(8)), 12737);
    }

    const PART_TWO_EXAMPLES: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn navigates_as_ghost() {
        let input = PART_TWO_EXAMPLES.trim_start().trim_end().lines().map(str::to_owned).collect();
        assert_eq!(total_steps_as_ghost(input), 6);
    }

    #[test]
    fn solves_part_two() {
        assert_eq!(total_steps_as_ghost(daily_input(8)), 9064949303801);
    }
}
