use crate::{day, utils::split2, EyreResult};
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

day! {
    parser,
    part1 => "There are {} paths",
    part2 => "There are {} paths",
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Cave<'a> {
    name: &'a str,
    small: bool,
}

type Parsed<'a> = (HashMap<&'a str, NodeIndex>, UnGraph<Cave<'a>, ()>);

pub(crate) fn parser(input: &str) -> EyreResult<Parsed<'_>> {
    let mut nodes = HashMap::new();
    let mut graph = UnGraph::new_undirected();
    let edges: Vec<_> = input
        .lines()
        .map(|line| -> EyreResult<_> {
            let (start, end) =
                split2(line, "-").ok_or(color_eyre::eyre::eyre!("Invalid line: {}", line))?;
            let start = Cave {
                name: start,
                small: start.chars().all(|c| c.is_lowercase()),
            };
            let end = Cave {
                name: end,
                small: end.chars().all(|c| c.is_lowercase()),
            };
            let start = *nodes
                .entry(start.name)
                .or_insert_with(|| graph.add_node(start));
            let end = *nodes.entry(end.name).or_insert_with(|| graph.add_node(end));
            Ok((start, end))
        })
        .collect::<EyreResult<_>>()?;
    graph.extend_with_edges(edges);
    Ok((nodes, graph))
}

#[derive(Clone)]
struct Path {
    nodes: Vec<NodeIndex>,
    small_twice: bool,
}

fn can_visit(
    node: NodeIndex,
    start: NodeIndex,
    path: &Path,
    graph: &UnGraph<Cave<'_>, ()>,
) -> bool {
    node != start && !is_small_and_visited(node, path, graph)
}

fn is_small_and_visited(node: NodeIndex, path: &Path, graph: &UnGraph<Cave<'_>, ()>) -> bool {
    if !path.nodes.contains(&node) {
        false
    } else {
        let label = graph.node_weight(node).unwrap();
        label.small
    }
}

fn can_visit_twice_once(
    node: NodeIndex,
    start: NodeIndex,
    path: &Path,
    graph: &UnGraph<Cave<'_>, ()>,
) -> bool {
    node != start && can_visit_if_small(node, path, graph)
}

fn can_visit_if_small(node: NodeIndex, path: &Path, graph: &UnGraph<Cave<'_>, ()>) -> bool {
    if !path.nodes.contains(&node) {
        true
    } else {
        let label = graph.node_weight(node).unwrap();

        if label.small {
            !path.small_twice
        } else {
            true
        }
    }
}

fn cave_paths<F: Fn(NodeIndex, NodeIndex, &Path) -> bool>(
    nodes: &HashMap<&str, NodeIndex>,
    graph: &UnGraph<Cave<'_>, ()>,
    visitable: F,
) -> EyreResult<Vec<Path>> {
    let start = *nodes
        .get("start")
        .ok_or(color_eyre::eyre::eyre!("No start"))?;
    let end = *nodes.get("end").ok_or(color_eyre::eyre::eyre!("No end"))?;

    let mut paths = vec![Path {
        nodes: vec![start],
        small_twice: false,
    }];

    loop {
        let mut new_paths = Vec::new();
        let mut stuck = true;

        for path in paths {
            // print_path(&path, &graph);
            let &last = path.nodes.last().unwrap();
            if last == end {
                new_paths.push(path);
            } else {
                stuck = false;
                graph
                    .neighbors(last)
                    .filter(|&neighbour| visitable(neighbour, start, &path))
                    .for_each(|end| {
                        let mut p = path.clone();
                        if p.nodes.contains(&end) && graph.node_weight(end).unwrap().small {
                            p.small_twice = true;
                        }
                        p.nodes.push(end);
                        new_paths.push(p);
                    })
            }
        }

        if stuck {
            break Ok(new_paths);
        } else {
            paths = new_paths;
        }
    }
}

pub(crate) fn part1((nodes, graph): Parsed) -> EyreResult<usize> {
    Ok(cave_paths(&nodes, &graph, |neighbour, start, path| {
        can_visit(neighbour, start, path, &graph)
    })?
    .len())
}

pub(crate) fn part2((nodes, graph): Parsed) -> EyreResult<usize> {
    Ok(cave_paths(&nodes, &graph, |neighbour, start, path| {
        can_visit_twice_once(neighbour, start, path, &graph)
    })?
    .len())
}
