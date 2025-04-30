//! main.rs
//! This is the main entry point. It loads the data, builds the graph, and runs analyses.

mod graph;
mod parser;
mod property;
mod analysis;
mod clustering;
mod knn;
mod heatmap;          
mod heatmap_draw;

use graph::Graph;
use parser::parse_csv;
use analysis::{analyze_graph, descriptive_statistics, run_cluster_analysis};
use heatmap::generate_heatmap;
use heatmap_draw::draw_heatmap;
use std::time::Instant;

fn main() {
    // Start the timer for the entire program
    let program_start = Instant::now();

    // Load properties from CSV
    let properties = parse_csv("hunnit_thau.csv");
    println!("Loaded {} properties.", properties.len());

    let mut graph = Graph::new();
    graph.load_properties(properties.clone());

    // TIMING
    let start = Instant::now();
    graph.add_edges_by_price_similarity(20000.0);  // use $20,000 threshold
    graph.add_edges_by_town();                      // Add town edges AFTER price similarity
    let duration = start.elapsed();
    println!("Graph building took: {:.2?}", duration);

    // Perform graph structure and KNN analysis
    analyze_graph(&graph);

    // Print descriptive statistics on sale prices
    descriptive_statistics(&graph);

    // Run KMeans cluster analysis
    println!("\nRunning Smart KMeans Cluster Analysis...");
    run_cluster_analysis(&properties);

    // Generate heatmaps
    generate_heatmap(&properties, "ct_heatmap.csv");
    draw_heatmap(&properties, "ct_heatmap.png");

    // End the timer for the entire program
    let program_duration = program_start.elapsed();
    println!(
        "\nFor {} properties, the program took {:.2?} to run.",
        properties.len(),
        program_duration
    );
}