//! clustering.rs
//! This module performs k-means clustering on property data based on key features.

use smartcore::cluster::kmeans::{KMeans, KMeansParameters};
use smartcore::linalg::basic::matrix::DenseMatrix;
use crate::property::Property;
use std::collections::HashMap;

/// Cluster structure: each property is assigned to a group (label).
/// Returns a vector of (cluster label, property reference) pairs.
pub fn kmeans_cluster(properties: &[Property], k: usize) -> Vec<(usize, &Property)> {
    // Prepare dataset matrix
    let mut data = vec![];
    let mut indexed: Vec<&Property> = vec![];

    let type_map: HashMap<String, f64> = properties
        .iter()
        .filter(|p| !p.property_type.is_empty())
        .map(|p| p.property_type.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, t)| (t, i as f64))
        .collect();

    for prop in properties.iter() {
        if prop.sale_amount > 0.0 && prop.assessed_value > 0.0 && prop.sales_ratio > 0.0 {
            let prop_type_encoded = type_map.get(&prop.property_type).cloned().unwrap_or(0.0);
            data.push(vec![
                prop.sale_amount,
                prop.assessed_value,
                prop.sales_ratio,
                prop_type_encoded,
            ]);
            indexed.push(prop);
        }
    }

    // Create a DenseMatrix
    let matrix = DenseMatrix::from_2d_vec(&data)
        .expect("Failed to create DenseMatrix from property features");

    // Fit KMeans model
    let model = KMeans::fit(&matrix, KMeansParameters::default().with_k(k))
        .expect("Failed to fit KMeans model");

    // Predict cluster labels
    let labels: Vec<usize> = model.predict(&matrix)
        .expect("Failed to predict clusters");

    labels.into_iter().zip(indexed).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::Property;

    #[test]
    fn test_kmeans_cluster_basic() {
        let properties = vec![
            Property {
                serial_number: "A".to_string(),
                year: 2020,
                date_recorded: "2020-01-01".to_string(),
                town: "TestTown".to_string(),
                address: "1 Test Street".to_string(),
                assessed_value: 500000.0,
                sale_amount: 520000.0,
                sales_ratio: 1.04,
                property_type: "Single Family".to_string(),
                residential_type: None,
                location: None,
            },
            Property {
                serial_number: "B".to_string(),
                year: 2020,
                date_recorded: "2020-01-01".to_string(),
                town: "TestTown".to_string(),
                address: "2 Test Street".to_string(),
                assessed_value: 505000.0,
                sale_amount: 515000.0,
                sales_ratio: 1.02,
                property_type: "Single Family".to_string(),
                residential_type: None,
                location: None,
            },
            Property {
                serial_number: "C".to_string(),
                year: 2020,
                date_recorded: "2020-01-01".to_string(),
                town: "TestTown".to_string(),
                address: "3 Test Street".to_string(),
                assessed_value: 1000000.0,
                sale_amount: 1100000.0,
                sales_ratio: 1.10,
                property_type: "Commercial".to_string(),
                residential_type: None,
                location: None,
            },
        ];

        let clustered = kmeans_cluster(&properties, 2);
        assert_eq!(clustered.len(), 3);
        assert!(clustered.iter().all(|(_, p)| p.sale_amount > 0.0));
    }
}
