use crate::graph::Graph;
use crate::property::Property;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq)]
struct Neighbor {
    distance: f64,
    property: Property,
}

impl Eq for Neighbor {}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

pub fn knn(graph: &Graph, target: &Property, k: usize) -> Vec<Property> {
    let mut neighbors = BinaryHeap::new();

    for property in graph.nodes.values() {
        if property.serial_number == target.serial_number {
            continue;
        }

        let distance = euclidean_distance(target, property);
        neighbors.push(Neighbor { distance, property: property.clone() });

        if neighbors.len() > k {
            neighbors.pop();
        }
    }

    neighbors.into_sorted_vec().iter().map(|n| n.property.clone()).collect()
}

fn euclidean_distance(p1: &Property, p2: &Property) -> f64 {
    let price_diff = (p1.sale_amount - p2.sale_amount).powi(2);
    let value_diff = (p1.assessed_value - p2.assessed_value).powi(2);
    
    (price_diff + value_diff).sqrt()
}
#[allow(dead_code)] // was using this initially however it was being problamatic 
pub fn run_knn(graph: &Graph) {
    println!("\n--- K-Nearest Neighbors Analysis ---");
    for target in graph.nodes.values() {
        let neighbors = knn(graph, target, 5);
        println!("Property: {} in {}", target.address, target.town);
        for neighbor in neighbors {
            println!("  Similar Property: {} in {}, Sold for ${}", 
                neighbor.address, neighbor.town, neighbor.sale_amount);
        }
    }
}