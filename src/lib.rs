#![no_std]
pub mod nand;
use petgraph::graph::Graph;
use petgraph::dot::Dot;
// use petgraph_evcxr::{draw_graph, draw_graph_with_attr_getters, draw_dot};
fn foo() {
    let mut g: Graph<&str, &str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    g.add_edge(a, b, "a to c");
    //draw_graph(&g);
}