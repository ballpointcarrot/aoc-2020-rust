use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Direction;
use regex::Regex;

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> DiGraph<String, usize> {
    let id_re = Regex::new("^(?P<color>\\D+) bags contain").unwrap();
    let rule_re = Regex::new("(?P<count>\\d+) (?P<color>\\D+) bag[s]?").unwrap();
    let mut bag_graph = DiGraph::<String, usize>::new();

    let rules: Vec<&str> = input.lines().collect();

    // Create graph nodes.
    let nodes: Vec<NodeIndex> = rules
        .iter()
        .map(|line| {
            bag_graph.add_node(String::from(
                id_re
                    .captures(line)
                    .unwrap()
                    .name("color")
                    .unwrap()
                    .as_str(),
            ))
        })
        .collect();

    // Connect graph nodes
    nodes.iter().for_each(|node| {
        let rule_str = rules.iter().find(|rule| {
            rule.contains(&format!(
                "{} bags contain",
                bag_graph.node_weight(*node).unwrap()
            ))
        });
        rule_re.captures_iter(rule_str.unwrap()).for_each(|mat| {
            let target_str = mat.name("color").unwrap().as_str();
            let edge_weight = str::parse(mat.name("count").unwrap().as_str())
                .expect("Couldn't build number from count!");
            let target_node = nodes
                .iter()
                .find(|n| bag_graph.node_weight(**n).unwrap() == target_str)
                .unwrap();
            bag_graph.add_edge(*node, *target_node, edge_weight);
        })
    });
    bag_graph
}

#[aoc(day7, part1)]
fn contains_bag(input: &DiGraph<String, usize>) -> usize {
    let mut flip = input.clone();
    flip.reverse();
    let shiny_gold_index = flip
        .node_indices()
        .find(|i| flip[*i] == "shiny gold")
        .unwrap();
    let mut count = 0;
    let mut dfs = Dfs::new(&flip, shiny_gold_index);
    while let Some(_) = dfs.next(&flip) {
        count += 1;
    }
    count - 1
}

#[aoc(day7, part2)]
fn total_bags(input: &DiGraph<String, usize>) -> usize {
    let shiny_gold_index = input
        .node_indices()
        .find(|i| input[*i] == "shiny gold")
        .unwrap();
    input
        .neighbors_directed(shiny_gold_index, Direction::Outgoing)
        .map(|node| edge_counts(input, shiny_gold_index, node))
        .sum()
}

fn edge_counts(graph: &DiGraph<String, usize>, parent: NodeIndex, node: NodeIndex) -> usize {
    let bag_count_edge = graph.find_edge(parent, node).unwrap();
    let bag_count = *(graph.edge_weight(bag_count_edge).unwrap());
    let neighbors = graph.neighbors_directed(node, Direction::Outgoing);
    let nested_count: usize = if neighbors.count() == 0 {
        0
    } else {
        graph.neighbors_directed(node, Direction::Outgoing).map(|n| bag_count * edge_counts(graph, node, n)).sum()
    };
    bag_count + nested_count
}

#[cfg(test)]
#[test]
fn test_regex() {
    let test_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let rule_re = Regex::new("(?P<count>\\d+) (?P<color>\\D+) bag[s]?").unwrap();
    rule_re
        .captures_iter(test_str)
        .for_each(|mat| println!("{:?}", mat));
    assert_eq!(1, 1);
}

#[test]
fn test_part1() {
    let test_str = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    assert_eq!(4, contains_bag(&parse_input_day7(test_str)));
}

#[test]
fn test_part2() {
    let test_str = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let test_str2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
    assert_eq!(32, total_bags(&parse_input_day7(test_str)));
    assert_eq!(126, total_bags(&parse_input_day7(test_str2)));
}

#[test]
fn test_sum() {
    assert_eq!(0, Vec::<i32>::new().iter().sum())
}
