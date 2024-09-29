use petgraph::graph::UnGraph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

fn main() {
    // Create an undirected graph
    let mut graph = UnGraph::new_undirected();

    // Define the number of vertices
    let n = 25;

    // Add n vertices
    let vertices: Vec<_> = (0..n).map(|i| graph.add_node(i + 1)).collect();

    // Make a loop to add edges to form a complete graph
    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            graph.add_edge(vertices[i], vertices[j], 0);
        }
    }

    // Generate the dot file content
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]);

    // Convert the dot content to a string
    let dot_content = format!("{}", dot);

    // Remove the extra 'graph { ... }' block
    let cleaned_dot_content = dot_content.replace("graph {", "").replace("}\n", "");

    // Add the 'layout=neato' and 'node [shape=circle]' directives at the top of the cleaned dot content
    let final_dot_content = format!(
        "graph G {{\nlayout=circo;\nnode [shape=circle];\n{}}}",
        cleaned_dot_content
    );

    // Write the final dot content to a file
    let mut file = File::create("graph.dot").expect("Unable to create file");
    write!(file, "{}", final_dot_content).expect("Unable to write data");

    println!("Dot file generated successfully.");
}
