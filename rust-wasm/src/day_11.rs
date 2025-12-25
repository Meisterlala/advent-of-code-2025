crate::solution!(
    11,
    "Reactor",
    r#"This is a classic <a href="https://en.wikipedia.org/wiki/Graph_theory">graph theory</a>
    problem. I model this as a <a href="https://en.wikipedia.org/wiki/Directed_acyclic_graph">DAG</a> and iterate over it in topological order while keeping track of connected nodes.

    For Part 2, we can break down the problem into two distinct paths that can be combined:
    $$\begin{align*}
        \text{svr} \rightarrow \text{dac} \rightarrow \text{fft} \rightarrow \text{out} \\
        \text{svr} \rightarrow \text{fft} \rightarrow \text{dac} \rightarrow \text{out}
    \end{align*} $$

    <div id="day11_example">
    <div class="label"><img src="day11_a.png" alt="Day 11 Part 1" id="day11_a"><span class="alt">Part 1</span></div>
    <div class="label"><img src="day11_b.png" alt="Day 11 Part 2" id="day11_b"><span class="alt">Part 2</span></div>
    </div>
    Technically the second path is not needed and could be removed. Because the input data doesn't have a connection $\text{dac} \rightarrow \text{fft}$.<br><a href="day11.png">You can see the full graph here</a>."#,
    &EXAMPLE,
    solve_a,
    solve_b
);

static EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

static EXAMPLE_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

use std::collections::{HashMap, HashSet};

use nom::{
    IResult, Parser,
    bytes::complete::take_while_m_n,
    character::complete::{char, multispace1, space0, space1},
    multi::separated_list1,
};

use petgraph::{
    dot::{Config, Dot},
    graph::DiGraph,
    visit::Topo,
};
use rayon::prelude::*;

pub fn parse(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    let device = || take_while_m_n(3, 3, |c: char| c.is_ascii_alphabetic());
    let connection = (
        device(),
        (char(':'), space0),
        separated_list1(space1, device()),
    )
        .map(|(from, _, tos)| (from, tos));
    let (rest, connections) = separated_list1(multispace1, connection).parse(input.trim())?;
    debug_assert!(rest.is_empty(), "Unparsed input remaining");
    debug_assert!(connections.len() > 1,);
    Ok((rest, connections))
}

fn connections_to_graph(connections: &Vec<(&str, Vec<&str>)>) -> DiGraph<String, ()> {
    let mut graph = DiGraph::<_, ()>::with_capacity(
        connections.len(),
        connections[0].1.len() * connections.len(),
    );

    let mut unique_nodes: HashSet<&str> = HashSet::new();
    for connection in connections {
        unique_nodes.insert(connection.0);
        for &to in &connection.1 {
            unique_nodes.insert(to);
        }
    }

    // Build graph + node map in one iterator pass
    let nodes: HashMap<&str, _> = unique_nodes
        .iter()
        .copied()
        .map(|s| (s, graph.add_node(s.to_owned())))
        .collect();

    // Add all edges in iterator style
    graph.extend_with_edges(
        connections
            .iter()
            .flat_map(|connection| connection.1.iter().map(move |&to| (connection.0, to)))
            .map(|(from, to)| {
                (
                    *nodes.get(from).unwrap(), // Safe: all nodes pre-added
                    *nodes.get(to).unwrap(),
                    (),
                )
            }),
    );

    graph
}

pub fn solve_a(input: &str) -> u64 {
    let (_, connections) = parse(input).expect("Failed to parse");
    let graph = connections_to_graph(&connections);
    connections_between(&graph, "you", "out")
}

#[allow(dead_code)]
fn save_dot(graph: &DiGraph<String, ()>, path: &str) {
    use std::fs::File;
    use std::io::Write;

    let fancy_dot = Dot::with_attr_getters(
        &graph,
        &[Config::EdgeNoLabel, Config::NodeNoLabel],
        &|_graph_reference, _edge_reference| String::new(),
        // Node attribute getter; We don't change any node attributes
        &|_, (_, label)| {
            let deco = match label.as_str() {
                "you" | "svr" => {
                    String::from(r#", shape = "box", style = "filled", fillcolor = "green""#)
                }
                "out" => String::from(r#", shape = "box", style = "filled", fillcolor = "red""#),
                "dac" | "fft" => String::from(
                    r#", shape = "diamond", style = "filled", fillcolor = "lightblue""#,
                ),
                _ => String::new(),
            };
            format!(r#"label = "{}"{}"#, label, deco)
        },
    );
    let mut file = File::create(path).expect("Unable to create file");
    write!(file, "{:?}", fancy_dot).expect("Unable to write data");
}

pub fn solve_b(input: &str) -> u64 {
    // Hack, so that example B can be tested with solve_a
    let inp = if input.trim() == EXAMPLE.trim() {
        EXAMPLE_B
    } else {
        input.trim()
    };
    let (_, connections) = parse(inp).expect("Failed to parse");
    let graph = connections_to_graph(&connections);

    let routes = [
        [("svr", "dac"), ("dac", "fft"), ("fft", "out")],
        [("svr", "fft"), ("fft", "dac"), ("dac", "out")],
    ];

    routes
        .into_par_iter()
        .map(|route| {
            route
                .into_par_iter()
                .map(|(from, to)| connections_between(&graph, from, to))
                .product::<u64>()
        })
        .sum::<u64>()
}

fn connections_between(graph: &DiGraph<String, ()>, from: &str, to: &str) -> u64 {
    let mut connection_map = vec![0u64; graph.node_count()];

    let from_index = graph
        .node_indices()
        .find(|&i| graph[i] == from)
        .expect(&format!("No '{}' node found", from));

    let to_index = graph
        .node_indices()
        .find(|&i| graph[i] == to)
        .expect(&format!("No '{}' node found", to));

    // Set 'from' node to 1
    connection_map[from_index.index()] = 1;

    // Traverse in graph
    let mut topo = Topo::new(&graph);
    while let Some(node_index) = topo.next(&graph) {
        let paths_to_node: u64 = graph
            .neighbors_directed(node_index, petgraph::Incoming)
            .map(|neighbor| connection_map[neighbor.index()])
            .sum();
        connection_map[node_index.index()] += paths_to_node;
    }

    connection_map[to_index.index()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let (remaining, parsed) = parse(EXAMPLE).expect("Failed to parse");
        assert!(remaining.is_empty(), "Unparsed input remaining");
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0].0, "aaa");
        assert_eq!(parsed[0].1, vec!["you", "hhh"]);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(EXAMPLE), 5);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(EXAMPLE), 2);
    }

    #[ignore]
    #[test]
    fn test_print_graphs() {
        let (_, connections) = parse(EXAMPLE).expect("Failed to parse");
        let graph = connections_to_graph(&connections);
        save_dot(&graph, "day11_a.dot");

        let (_, connections_b) = parse(EXAMPLE_B).expect("Failed to parse");
        let graph_b = connections_to_graph(&connections_b);
        save_dot(&graph_b, "day11_b.dot");
    }
}
