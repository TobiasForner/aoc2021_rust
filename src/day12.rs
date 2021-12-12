use crate::util::read_to_vec;
use anyhow::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Node {
    Small(String),
    Large(String),
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Node::*;
        if s.chars().all(|c| matches!(c, 'a'..='z')) {
            Ok(Small(s.to_string()))
        } else {
            Ok(Large(s.to_string()))
        }
    }
}

#[derive(Clone)]
struct Edge {
    start: Node,
    end: Node,
}

impl FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        let start = Node::from_str(parts[0])?;
        let end = Node::from_str(parts[1])?;
        Ok(Edge { start, end })
    }
}

fn parse_input(path: &str) -> Result<HashMap<Node, Vec<Edge>>> {
    let edges: Vec<Edge> = read_to_vec(path)?;
    let mut node_to_edges: HashMap<Node, Vec<Edge>> = HashMap::new();
    let rev_edges: Vec<Edge> = edges
        .iter()
        .map(|e| Edge {
            start: e.end.clone(),
            end: e.start.clone(),
        })
        .collect();
    for e in edges {
        let start = e.start.clone();
        node_to_edges.entry(start).or_insert(vec![]).push(e);
    }
    for e in rev_edges {
        let start = e.start.clone();
        node_to_edges.entry(start).or_insert(vec![]).push(e);
    }
    Ok(node_to_edges)
}

fn explore(
    visited: HashSet<Node>,
    edges: &HashMap<Node, Vec<Edge>>,
    current: &Node,
    small_visited_twice: bool,
) -> u64 {
    use Node::*;
    let mut result = 0;
    macro_rules! visit_node {
        ($e: expr, $visited_twice: expr) => {
            let mut new_visited = visited.clone();
            new_visited.insert($e.end.clone());
            result += explore(new_visited, edges, &$e.end, $visited_twice);
        };
    }
    if let Some(curr_edges) = edges.get(current) {
        for e in curr_edges {
            match &e.end {
                Small(x) => {
                    if x == "end" {
                        result += 1;
                    } else if x == "start" {
                        continue;
                    } else if visited.contains(&e.end) {
                        if !small_visited_twice {
                            visit_node!(e, true);
                        } else {
                            continue;
                        }
                    } else {
                        visit_node!(e, small_visited_twice);
                    }
                }
                Large(_) => {
                    visit_node!(e, small_visited_twice);
                }
            }
        }
    }
    result
}

pub fn part1(path: &str) -> Result<u64> {
    let node_to_edges = parse_input(path)?;
    let start = Node::Small("start".to_string());
    let mut visited = HashSet::new();
    visited.insert(start.clone());
    let res = explore(visited, &node_to_edges, &start, true);
    Ok(res)
}

pub fn part2(path: &str) -> Result<u64> {
    let node_to_edges = parse_input(path)?;
    let start = Node::Small("start".to_string());
    let mut visited = HashSet::new();
    visited.insert(start.clone());
    let res = explore(visited, &node_to_edges, &start, false);
    Ok(res)
}
