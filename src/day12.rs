use crate::util::{read_to_vec_ctx, FromStrCtx};
use anyhow::{Context, Error, Result};
use std::collections::HashMap;

type Ctx = HashMap<String, Node>;
type NodeLabel = usize;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
enum Node {
    Small(NodeLabel),
    Large(NodeLabel),
}

impl Node {
    fn label(&self) -> NodeLabel {
        match self {
            Node::Small(l) | Node::Large(l) => *l,
        }
    }
}

impl FromStrCtx<Ctx> for Node {
    type Err = Error;

    fn from_str_ctx(s: &str, labels: &mut Ctx) -> Result<Self> {
        use Node::*;

        if let Some(node) = labels.get(s) {
            Ok(*node)
        } else {
            let label: NodeLabel = labels.values().map(|x| x.label()).max().unwrap_or(0) + 1;

            let node = if s.chars().all(|c| matches!(c, 'a'..='z')) {
                Small(label)
            } else {
                Large(label)
            };
            labels.insert(s.to_string(), node);
            Ok(node)
        }
    }
}

#[derive(Clone)]
struct Edge {
    start: Node,
    end: Node,
}

impl FromStrCtx<Ctx> for Edge {
    type Err = Error;

    fn from_str_ctx(s: &str, labels: &mut Ctx) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        let start = Node::from_str_ctx(parts[0], labels)?;
        let end = Node::from_str_ctx(parts[1], labels)?;
        Ok(Edge { start, end })
    }
}

fn parse_input_vec(path: &str) -> Result<(Vec<Vec<Edge>>, NodeLabel, NodeLabel)> {
    let mut labels = HashMap::new();
    let edges: Vec<Edge> = read_to_vec_ctx(path, &mut labels)?;
    let mut node_to_edges = vec![Vec::with_capacity(10); labels.len() + 1];
    let rev_edges = edges.iter().map(|e| Edge {
        start: e.end.clone(),
        end: e.start.clone(),
    });
    for e in rev_edges {
        let start = e.start.clone();
        node_to_edges[start.label()].push(e);
    }
    for e in edges {
        let start = e.start.clone();
        node_to_edges[start.label()].push(e);
    }
    let start = labels.get("start").context("No start node!")?.label();
    let end = labels.get("end").context("No end found")?.label();
    Ok((node_to_edges, start, end))
}

fn explore_vec(
    visited: Vec<bool>,
    edges: &Vec<Vec<Edge>>,
    current: &Node,
    small_visited_twice: bool,
    start: NodeLabel,
    end: NodeLabel,
) -> u64 {
    use Node::*;
    let mut result = 0;
    macro_rules! visit_node {
        ($e: expr, $visited_twice: expr) => {
            let mut new_visited = visited.clone();
            new_visited[$e.end.label()] = true;
            result += explore_vec(new_visited, edges, &$e.end, $visited_twice, start, end);
        };
    }
    let curr_edges = &edges[current.label()];
    for e in curr_edges {
        match &e.end {
            Small(x) => {
                if *x == end {
                    result += 1;
                } else if *x == start {
                    continue;
                } else if visited[e.end.label()] {
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
    result
}

pub fn part1(path: &str) -> Result<u64> {
    let (node_to_edges, start, end) = parse_input_vec(path)?;
    let start_node = Node::Small(start);
    let mut visited = vec![false; node_to_edges.len()];
    visited[start as usize] = true;

    let res = explore_vec(visited, &node_to_edges, &start_node, true, start, end);
    Ok(res)
}

pub fn part2(path: &str) -> Result<u64> {
    let (node_to_edges, start, end) = parse_input_vec(path)?;
    let start_node = Node::Small(start);
    let mut visited = vec![false; node_to_edges.len()];
    visited[start as usize] = true;
    let res = explore_vec(visited, &node_to_edges, &start_node, false, start, end);
    Ok(res)
}
