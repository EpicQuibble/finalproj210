//! graph.rs
//! This module defines the Graph structure, allowing properties to be connected based on town or other features.

use std::collections::{HashMap, HashSet};
use crate::property::Property;
use rand::prelude::*;
use std::sync::Mutex;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;  
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Represents a graph where nodes are properties and edges represent connections (e.g., same town).
#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Property>,
    pub edges: HashMap<String, HashSet<String>>, // Adjacency list representation
}

impl Graph {

    pub fn load_properties(&mut self, properties: Vec<Property>) {
        for property in properties {
            self.nodes.insert(property.serial_number.clone(), property);
        }
    }

    /// Create a new, empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    /// Add a property node to the graph.
    pub fn add_property(&mut self, property: Property) {
        self.nodes.insert(property.serial_number.clone(), property);
    }
    
    /// Add an undirected edge between two property nodes.
    pub fn add_edge(&mut self, id1: &str, id2: &str) {
        self.edges.entry(id1.to_string()).or_default().insert(id2.to_string());
        self.edges.entry(id2.to_string()).or_default().insert(id1.to_string());
    }

    /// Add edges between properties that are located in the same town.
    
    pub fn add_edges_by_town(&mut self) {
        let mut town_map: HashMap<String, Vec<String>> = HashMap::new();

        // Group property serial numbers by town
        for property in self.nodes.values() {
            town_map
                .entry(property.town.clone())
                .or_default()
                .push(property.serial_number.clone());
        }

        let mut rng = StdRng::seed_from_u64(69); // Number here is the seed -> 69        // For each town group, connect properties randomly
        for properties in town_map.values() {
            if properties.len() <= 5 {
                // If small group, fully connect
                for (i, id1) in properties.iter().enumerate() {
                    for id2 in properties.iter().skip(i + 1) {
                        self.add_edge(id1, id2);
                    }
                }
            } else {
                // If large group, connect each property to 3 random others
                for id1 in properties {
                    let others: Vec<&String> = properties.iter().filter(|id2| *id2 != id1).collect();
                    let sample = others.choose_multiple(&mut rng, 3); // Connect to 3 random others
                    for id2 in sample {
                        self.add_edge(id1, id2);
                    }
                }
            }
        }

        // Ensure all properties have at least an entry
        for property in self.nodes.keys() {
            self.edges.entry(property.clone()).or_default();
        }
    }
    

    pub fn add_edges_by_price_similarity(&mut self, threshold: f64) {
        let mut properties: Vec<_> = self.nodes.values().cloned().collect();
        properties.sort_by(|a, b| a.sale_amount.partial_cmp(&b.sale_amount).unwrap());
    
        let edges = Mutex::new(HashMap::<String, HashSet<String>>::new());
    
        (&properties).par_iter().for_each(|prop1| {
            for prop2 in &properties {
                let price_diff = prop2.sale_amount - prop1.sale_amount;
        
                if price_diff > threshold {
                    break; // 🔥 Early exit if price difference is too large
                }
        
                if price_diff.abs() <= threshold {
                    let mut edges_guard = edges.lock().unwrap();
                    edges_guard.entry(prop1.serial_number.clone())
                        .or_insert_with(HashSet::new)
                        .insert(prop2.serial_number.clone());
                    edges_guard.entry(prop2.serial_number.clone())
                        .or_insert_with(HashSet::new)
                        .insert(prop1.serial_number.clone());
                }
            }
        });
        
    
        self.edges = edges.into_inner().unwrap();
    }
        
    /// Return the number of nodes (properties) in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of edges (connections) in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
    }

    /// Calculate the degree distribution of the graph (how many nodes have each degree).
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for neighbors in self.edges.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }
        distribution
    }

    /// Return an iterator over the properties (nodes).
    pub fn nodes(&self) -> impl Iterator<Item = &Property> {
        self.nodes.values()
    }

    /// Simple clustering by sale amount: split into `k` groups by price.
    pub fn cluster_properties(&self, k: usize) -> Vec<Vec<&Property>> {
        let mut clusters: Vec<Vec<&Property>> = vec![vec![]; k];
        for property in self.nodes.values() {
            let cluster_index = (property.sale_amount / 500_000.0)
                .min(k as f64 - 1.0) as usize;
            clusters[cluster_index].push(property);
        }
        clusters
    }
}
