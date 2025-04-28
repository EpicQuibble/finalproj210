use std::collections::{HashMap, HashSet};
use crate::property::Property;

#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Property>,
    pub edges: HashMap<String, HashSet<String>>, // Adjacency list
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    pub fn add_property(&mut self, property: Property) {
        self.nodes.insert(property.serial_number.clone(), property);
    }
    
    pub fn add_edge(&mut self, id1: &str, id2: &str) {
        self.edges.entry(id1.to_string()).or_default().insert(id2.to_string());
        self.edges.entry(id2.to_string()).or_default().insert(id1.to_string());
    }

    /// Adds edges between properties in the same town
    pub fn add_edges_by_town(&mut self) {
        let mut town_map: HashMap<String, Vec<String>> = HashMap::new();
    
        // Group properties by town
        for property in self.nodes.values() {
            town_map
                .entry(property.town.clone())
                .or_default()
                .push(property.serial_number.clone());
        }
    
        // Add edges between properties in the same town
        for properties in town_map.values() {
            for (i, id1) in properties.iter().enumerate() {
                for id2 in properties.iter().skip(i + 1) {
                    self.add_edge(id1, id2);
                }
            }
        }
    
        // Ensure all properties have an entry in the edges map
        for property in self.nodes.keys() {
            self.edges.entry(property.clone()).or_default();
        }
    }

    /// Count the total number of nodes (properties)
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Count the total number of edges (connections)
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
    }

    /// Calculate the degree distribution
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for neighbors in self.edges.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }
        distribution
    }

    /// Return an iterator over the nodes (properties)
    pub fn nodes(&self) -> impl Iterator<Item = &Property> {
        self.nodes.values()
    }

    pub fn cluster_properties(&self, k: usize) -> Vec<Vec<&Property>> {
        let mut clusters: Vec<Vec<&Property>> = vec![vec![]; k];
        for property in self.nodes.values() {
            let cluster_index = (property.sale_amount / 500_000.0).min(k as f64 - 1.0) as usize;
            clusters[cluster_index].push(property);
        }
        clusters
    }
}