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




#[test]
fn test_parse_csv_with_mock_data() {
    use std::fs::{write, remove_file};
    use std::path::Path;

    // Header + one row of real-world-style data
    let content = "Serial Number,List Year,Date Recorded,Town,Address,Assessed Value,Sale Amount,Sales Ratio,Property Type,Residential Type,Non Use Code,Assessor Remarks,OPM remarks,Location,Longitude,Latitude\n\
220008,2022,01/30/2023,Andover,618 ROUTE 6,139020.00,232000.00,0.5992,Residential,Single Family,,,,POINT (-72.343628962 41.728431984),-72.343628962,41.728431984";

    let test_file = "test.csv";
    write(test_file, content).expect("Failed to write test CSV");

    let props = crate::parser::parse_csv(test_file);
    assert_eq!(props.len(), 1, "Expected one property, got: {:?}", props);

    let p = &props[0];
    assert_eq!(p.serial_number, "220008");
    assert_eq!(p.town, "Andover");
    assert_eq!(p.sale_amount, 232000.0);
    assert_eq!(p.sales_ratio, 0.5992);
    assert_eq!(p.location, Some((-72.343628962, 41.728431984)));

    if Path::new(test_file).exists() {
        remove_file(test_file).unwrap();
    }
}
