mod graph;
mod parser;
mod property;
mod analysis;
mod clustering;
mod knn;

use graph::Graph;
use parser::parse_csv;
use analysis::{analyze_graph, descriptive_statistics}; // Import descriptive_statistics

fn main() {
    // Parse the CSV file to load properties
    let properties = parse_csv("miniture_five_thou_dataset.csv");
    let mut graph = Graph::new();

    // Add properties to the graph
    for property in properties {
        graph.add_property(property);
    }

    // Add edges between properties in the same town
    graph.add_edges_by_town();

    // Perform graph analysis, including KNN
    analyze_graph(&graph);
    descriptive_statistics(&graph);
}