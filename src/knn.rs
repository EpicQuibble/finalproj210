//! knn.rs
//! This module implements K-Nearest Neighbors (KNN) to find similar properties based on multiple property features.

use crate::graph::Graph;
use crate::property::Property;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Represents a neighbor candidate with a distance to the target property.
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

/// Find the k-nearest properties to a given target property.
/// KNN is based on a composite distance of sale amount, assessed value, sales ratio, and property type encoding.
///
/// # Inputs
/// - `graph`: the property graph
/// - `target`: the property to compare against
/// - `k`: number of neighbors to find
///
/// # Outputs
/// - A list of `k` most similar `Property` instances
pub fn knn(graph: &Graph, target: &Property, k: usize) -> Vec<Property> {
    let mut neighbors = BinaryHeap::new();

    for property in graph.nodes.values() {
        if property.serial_number == target.serial_number {
            continue;
        }

        let distance = composite_distance(target, property);
        neighbors.push(Neighbor { distance, property: property.clone() });

        if neighbors.len() > k {
            neighbors.pop();
        }
    }

    neighbors.into_sorted_vec().iter().map(|n| n.property.clone()).collect()
}

/// Compute the Euclidean distance between two properties based on key features.
///
/// Features used:
/// - Sale amount
/// - Assessed value
/// - Sales ratio
/// - Encoded property type (as hash value)
fn composite_distance(p1: &Property, p2: &Property) -> f64 {
    let price_diff = (p1.sale_amount - p2.sale_amount).powi(2);
    let value_diff = (p1.assessed_value - p2.assessed_value).powi(2);
    let ratio_diff = (p1.sales_ratio - p2.sales_ratio).powi(2);
    let type_diff = (property_type_encoding(&p1.property_type) - property_type_encoding(&p2.property_type)).powi(2);

    (price_diff + value_diff + ratio_diff + type_diff).sqrt()
}

/// Encode property type into a simple numeric value.
/// This allows categorical comparison without deep ML encoding.
fn property_type_encoding(property_type: &str) -> f64 {
    let mut value = 0u64;
    for byte in property_type.bytes() {
        value = value.wrapping_mul(31).wrapping_add(byte as u64);
    }
    (value % 1000) as f64
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::property::Property;

    #[test]
    fn test_knn_basic() {
        let mut graph = Graph::new();

        let property1 = Property {
            serial_number: "1".to_string(),
            year: 2020,
            date_recorded: "2020-01-01".to_string(),
            town: "TestTown".to_string(),
            address: "123 A St".to_string(),
            assessed_value: 500000.0,
            sale_amount: 520000.0,
            sales_ratio: 1.04,
            property_type: "Single Family".to_string(),
            residential_type: None,
            location: None,
        };

        let property2 = Property {
            serial_number: "2".to_string(),
            year: 2021,
            date_recorded: "2021-01-01".to_string(),
            town: "TestTown".to_string(),
            address: "456 B St".to_string(),
            assessed_value: 510000.0,
            sale_amount: 530000.0,
            sales_ratio: 1.04,
            property_type: "Single Family".to_string(),
            residential_type: None,
            location: None,
        };

        let property3 = Property {
            serial_number: "3".to_string(),
            year: 2022,
            date_recorded: "2022-01-01".to_string(),
            town: "TestTown".to_string(),
            address: "789 C St".to_string(),
            assessed_value: 600000.0,
            sale_amount: 650000.0,
            sales_ratio: 1.08,
            property_type: "Condo".to_string(),
            residential_type: None,
            location: None,
        };

        graph.add_property(property1.clone());
        graph.add_property(property2.clone());
        graph.add_property(property3.clone());

        let neighbors = knn(&graph, &property1, 2);

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.iter().any(|p| p.serial_number == "2"));
    }
}
