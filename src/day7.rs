use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::visit::Dfs;
use regex::Regex;

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> DiGraph<String, usize> {
    let id_re = Regex::new("^(?P<color>\\D+) bags contain").unwrap();
    let rule_re = Regex::new("(?P<count>\\d+) (?P<color>\\D+) bag[s]?").unwrap();
    let mut bag_graph = DiGraph::<String, usize>::new();

    let rules: Vec<&str> = input.lines().collect();

    // Create graph nodes.
    let nodes: Vec<NodeIndex> = rules.iter().map(|line| {
        bag_graph.add_node(String::from(id_re.captures(line).unwrap().name("color").unwrap().as_str()))
    }).collect();

    // Connect graph nodes
    nodes.iter().for_each(|node| {
        let rule_str = rules.iter().find(|rule| rule.contains(&format!("{} bags contain", bag_graph.node_weight(*node).unwrap())));
        rule_re.captures_iter(rule_str.unwrap()).for_each(|mat| {
            let target_str = mat.name("color").unwrap().as_str();
            let edge_weight = str::parse(mat.name("count").unwrap().as_str()).expect("Couldn't build number from count!");
            let target_node = nodes.iter().find(|n| bag_graph.node_weight(**n).unwrap() == target_str).unwrap();
            println!("{} contain {} {} bag(s)", bag_graph[*node], bag_graph[*target_node], edge_weight);
            bag_graph.add_edge(*node, *target_node, edge_weight);
        })
    });

    let mut search = bag_graph.clone();
    search.reverse();
    search
}

#[aoc(day7, part1)]
fn contains_bag(input: &DiGraph<String, usize>) -> usize {
    let shiny_gold_index = input.node_indices().find(|i| input[*i] == "shiny gold").unwrap();
    let mut count = 0;
    let mut dfs = Dfs::new(input, shiny_gold_index);
    while let Some(node) = dfs.next(input) {
        count += 1;
    }
    count - 1
}

#[cfg(test)]

#[test]
fn test_regex() {
    let test_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let rule_re = Regex::new("(?P<count>\\d+) (?P<color>\\D+) bag[s]?").unwrap();
    rule_re.captures_iter(test_str).for_each(|mat| {
        println!("{:?}", mat)
    });
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
