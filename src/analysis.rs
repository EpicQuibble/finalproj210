//! analysis.rs
//! This module analyzes the property graph, computes statistics, and performs clustering and KNN.

use crate::graph::Graph;
use crate::knn::knn;
use crate::clustering::kmeans_cluster;
use crate::property::Property;
use std::collections::HashMap;

/// Analyze the graph structure: degrees, connections, and property counts.
pub fn analyze_graph(graph: &Graph) {
    println!("--- Graph Analysis ---");

    print_basic_graph_stats(graph);
    print_top_connected_properties(graph);
    print_high_value_properties(graph);
    print_top_towns(graph);
    print_knn_samples(graph);
    print_cluster_summary(graph);
}

/// Prints basic graph statistics (total nodes, edges, degrees)
fn print_basic_graph_stats(graph: &Graph) {
    let total_properties = graph.node_count();
    let total_connections = graph.edge_count();
    println!("Total Properties: {}", total_properties);
    println!("Total Connections: {}", total_connections);

    let degree_distribution = graph.degree_distribution();
    let max_degree = degree_distribution.keys().max().unwrap_or(&0);
    let avg_degree = degree_distribution
        .iter()
        .map(|(degree, count)| degree * count)
        .sum::<usize>() as f64
        / total_properties as f64;

    println!("\nDegree Summary:");
    println!("  Max Degree: {}", max_degree);
    println!("  Average Degree: {:.2}", avg_degree);
}

/// Print the top 5 most connected properties.
fn print_top_connected_properties(graph: &Graph) {
    let mut property_degrees: Vec<(&Property, usize)> = graph.nodes()
        .map(|property| {
            let degree = graph.edges.get(&property.serial_number).map_or(0, |neighbors| neighbors.len());
            (property, degree)
        })
        .collect();

    property_degrees.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop 5 Most Connected Properties:");
    for (property, degree) in property_degrees.iter().take(5) {
        println!("  {} in {} ({} connections)", property.address, property.town, degree);
    }
}

/// Print the top 5 highest-value property sales.
fn print_high_value_properties(graph: &Graph) {
    let mut high_value_properties = graph.nodes()
        .filter(|property| property.sale_amount > 1_000_000.0)
        .collect::<Vec<_>>();

    high_value_properties.sort_by(|a, b| b.sale_amount.partial_cmp(&a.sale_amount).unwrap());

    println!("\nTop 5 High-Value Properties (Above $1M):");
    for property in high_value_properties.iter().take(5) {
        println!("  {} in {} sold for ${}", property.address, property.town, property.sale_amount);
    }
}

/// Print the top 5 towns by property count.
fn print_top_towns(graph: &Graph) {
    let mut town_counts: HashMap<String, usize> = HashMap::new();
    for property in graph.nodes() {
        *town_counts.entry(property.town.clone()).or_insert(0) += 1;
    }

    let mut town_counts_vec: Vec<(&String, usize)> = town_counts.iter().map(|(town, &count)| (town, count)).collect();
    town_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop 5 Towns by Property Count:");
    for (town, count) in town_counts_vec.iter().take(5) {
        println!("  {}: {} properties", town, count);
    }
}

/// Run KNN on sample properties and print similar properties.
fn print_knn_samples(graph: &Graph) {
    println!("\n--- K-Nearest Neighbors Analysis ---");

    let sample_properties = graph.nodes().take(3).collect::<Vec<_>>();
    for property in sample_properties {
        println!("Property: {} in {}", property.address, property.town);
        let neighbors = knn(graph, property, 3);
        for neighbor in neighbors {
            println!("  Similar Property: {} in {}, Sold for ${}", neighbor.address, neighbor.town, neighbor.sale_amount);
        }
    }

    println!("\nKNN Insights:");
    println!("  High-value properties tend to have neighbors with similar sale amounts.");
}

/// Perform simple cluster analysis and print cluster summaries.
fn print_cluster_summary(graph: &Graph) {
    let clusters = graph.cluster_properties(3);
    println!("\nCluster Analysis:");
    for (i, cluster) in clusters.iter().enumerate() {
        let avg_price = cluster.iter().map(|p| p.sale_amount).sum::<f64>() / cluster.len() as f64;
        let common_type = cluster
            .iter()
            .filter_map(|p| Some(&p.property_type))
            .fold(HashMap::new(), |mut acc, t| {
                *acc.entry(t).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(t, _)| t.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        println!(
            "  Cluster {}: {} properties, Average Price: ${:.2}, Common Type: {}",
            i + 1,
            cluster.len(),
            avg_price,
            common_type
        );
    }
}

/// Compute descriptive statistics over sale amounts.
pub fn descriptive_statistics(graph: &Graph) {
    let sale_amounts: Vec<f64> = graph.nodes().map(|p| p.sale_amount).collect();
    let mean = sale_amounts.iter().sum::<f64>() / sale_amounts.len() as f64;

    let median = {
        let mut sorted = sale_amounts.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() / 2]
    };

    let std_dev = (sale_amounts.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / sale_amounts.len() as f64).sqrt();

    println!("\nDescriptive Statistics:");
    println!("  Mean Sale Amount: ${:.2}", mean);
    println!("  Median Sale Amount: ${:.2}", median);
    println!("  Standard Deviation: ${:.2}", std_dev);

    let outliers = sale_amounts
        .iter()
        .filter(|&&amount| (amount - mean).abs() > 2.0 * std_dev)
        .collect::<Vec<_>>();

    println!("  Outliers (2 Std Dev): {} properties", outliers.len());
}

/// Run k-means clustering and print cluster summaries.
pub fn run_cluster_analysis(properties: &[Property]) {
    let clustered = kmeans_cluster(properties, 3);
    let mut clusters: HashMap<usize, Vec<&Property>> = HashMap::new();

    for (label, prop) in clustered {
        clusters.entry(label).or_default().push(prop);
    }

    println!("\n--- KMeans Cluster Summary ---");
    for (label, props) in clusters {
        let count = props.len();
        let avg_sale = props.iter().map(|p| p.sale_amount).sum::<f64>() / count as f64;
        let avg_assessed = props.iter().map(|p| p.assessed_value).sum::<f64>() / count as f64;
        let avg_ratio = props.iter().map(|p| p.sales_ratio).sum::<f64>() / count as f64;

        let mut type_counts: HashMap<&str, usize> = HashMap::new();
        for p in props {
            *type_counts.entry(p.property_type.as_str()).or_insert(0) += 1;
        }

        let common_type = type_counts.into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
            .unwrap_or("Unknown");

        println!(
            "Cluster {}: {} properties, Avg Sale: ${:.2}, Avg Assessment: ${:.2}, Avg Ratio: {:.2}, Common Type: {}",
            label, count, avg_sale, avg_assessed, avg_ratio, common_type
        );
    }
}
