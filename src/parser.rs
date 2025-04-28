use csv::ReaderBuilder;
use rayon::prelude::*;
use crate::property::Property;
pub fn parse_csv(filename: &str) -> Vec<Property> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // Skip the header row
        .from_path(filename)
        .expect("Error: Cannot open file. Check path and permissions.");

    reader
        .records()
        .par_bridge() // Parallel iterator
        .filter_map(|result| {
            if let Ok(record) = result {
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
                    residential_type: None,
                    location: None,
                })
            } else {
                None
            }
        })
        .collect()
}