//! parser.rs
//! This module loads and parses the property dataset from a CSV file into property structs.

use csv::ReaderBuilder;
use rayon::prelude::*;
use crate::property::Property;

/// Parses given CSV file into a vector of property structs
///
/// Inputs -> filename, path to the CSV file to read
/// Outputs -> A vector of property structs populated with values from the CSV
///
/// High-level Logic :)
///  - Uses a CSV reader to parse each row
///  - Uses Rayon for parallel processing
///  - Filters and maps each CSV record into property struct
///  - Extracts location (longitude and latitude) when available

pub fn parse_csv(filename: &str) -> Vec<Property> {
    // Set up the CSV reader with headers enabled
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(filename)
        .expect("Error: Cannot open file. Check the path and permissions.");

    reader
        .records()              // Iterator over CSV rows
        .par_bridge()           // Enables parallel processing via Rayon
        .filter_map(|result| {
            if let Ok(record) = result {
                // Parse location coordinates if available
                let longitude = record.get(14)?.parse::<f64>().ok();
                let latitude = record.get(15)?.parse::<f64>().ok();

                // Construct and return a Property struct
                Some(Property {
                    serial_number: record.get(0)?.to_string(),
                    year: record.get(1)?.parse().unwrap_or(0),
                    date_recorded: record.get(2)?.to_string(),
                    town: record.get(3)?.to_string(),
                    address: record.get(4)?.to_string(),
                    assessed_value: record.get(5)?.parse().unwrap_or(0.0),
                    sale_amount: record.get(6)?.parse().unwrap_or(0.0),
                    sales_ratio: record.get(7)?.parse().unwrap_or(0.0),
                    property_type: record.get(8)?.to_string(),
                    residential_type: None, // Not parsed in this version
                    location: match (longitude, latitude) {
                        (Some(lon), Some(lat)) => Some((lon, lat)),
                        _ => None,
                    },
                })
            } else {
                None // Skip malformed records
            }
        })
        .collect()
}