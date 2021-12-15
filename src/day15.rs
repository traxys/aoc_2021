use crate::{day, EyreResult};
use petgraph::{algo::dijkstra, graph::UnGraph, visit::EdgeRef};

day! {
    parser,
    part1 => "{}",
    part2 => "{}",
}

type Parsed<'a> = &'a str;

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    Ok(input)
}

pub(crate) fn part1(input: Parsed) -> EyreResult<u64> {
    let mut lines = input.lines().map(|l| l.trim().as_bytes());
    let first = lines
        .next()
        .ok_or(color_eyre::eyre::eyre!("No first line"))?;
    let mut g: UnGraph<u8, ()> = UnGraph::with_capacity(input.len(), 2 * input.len());

    let start = g.add_node(first[0] - b'0');

    let mut curr_line = vec![start];
    for &n in &first[1..] {
        let node = g.add_node(n - b'0');
        g.add_edge(*curr_line.last().unwrap(), node, ());
        curr_line.push(node);
    }

    let mut prev_line = curr_line;
    for line in lines {
        let mut curr_line = Vec::with_capacity(line.len());
        let first = g.add_node(line[0] - b'0');
        curr_line.push(first);
        g.add_edge(prev_line[0], first, ());

        for (&n, &top) in line.iter().skip(1).zip(prev_line.iter().skip(1)) {
            let node = g.add_node(n - b'0');
            g.add_edge(*curr_line.last().unwrap(), node, ());
            g.add_edge(top, node, ());
            curr_line.push(node);
        }

        prev_line = curr_line;
    }

    let end = *prev_line.last().unwrap();

    let paths = dijkstra(&g, start, Some(end), |edge| g[edge.target()] as u64);
    let &end_cost = paths
        .get(&end)
        .ok_or(color_eyre::eyre::eyre!("End was not reachable"))?;

    Ok(end_cost)
}

fn pseudo_mod(y: u8, off: u8) -> u8 {
    let x = (y + off) % 10;
    (y + off > 9) as u8 + x
}

fn repeated_chain_skip_first(line: &[u8], row: u8) -> impl Iterator<Item = u8> + '_ {
    line[1..]
        .iter()
        .map(move |n| pseudo_mod(n - b'0', row))
        .chain(
            (1..5)
                .map(move |repeat| line.iter().map(move |n| pseudo_mod(n - b'0', row + repeat)))
                .flatten(),
        )
}

pub(crate) fn part2(input: Parsed) -> EyreResult<u64> {
    let mut lines = input.lines().map(|l| l.trim().as_bytes());
    let first = lines
        .next()
        .ok_or(color_eyre::eyre::eyre!("No first line"))?;
    let mut g: UnGraph<u8, ()> = UnGraph::with_capacity(input.len() * 25, 2 * 25 * input.len());

    let start = g.add_node(first[0] - b'0');

    print!("{}", first[0] - b'0');
    let mut curr_line = vec![start];
    for n in repeated_chain_skip_first(first, 0) {
        print!("{}", n);
        let node = g.add_node(n);
        g.add_edge(*curr_line.last().unwrap(), node, ());
        curr_line.push(node);
    }
    println!();

    let mut prev_line = curr_line;
    for repeat in 0..5 {
        for line in input
            .lines()
            .skip((repeat == 0) as usize)
            .map(|l| l.trim().as_bytes())
        {
            let n = pseudo_mod(line[0] - b'0', repeat);
            print!("{}", n);

            let mut curr_line = Vec::with_capacity(line.len() * 5);
            let first = g.add_node(n);
            curr_line.push(first);
            g.add_edge(prev_line[0], first, ());

            for (n, &top) in repeated_chain_skip_first(line, repeat).zip(prev_line.iter()) {
                let node = g.add_node(n);
                print!("{}", n);
                g.add_edge(*curr_line.last().unwrap(), node, ());
                g.add_edge(top, node, ());
                curr_line.push(node);
            }

            println!();
            prev_line = curr_line;
        }
    }

    let end = *prev_line.last().unwrap();

    let paths = dijkstra(&g, start, Some(end), |edge| g[edge.target()] as u64);
    let &end_cost = paths
        .get(&end)
        .ok_or(color_eyre::eyre::eyre!("End was not reachable"))?;

    Ok(end_cost)
}
