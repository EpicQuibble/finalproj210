//! property.rs
//! This module defines the Property struct to represent a real estate record.

/// Represents a single property record from the dataset.
/// Holds sale information, location, and basic attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub serial_number: String,
    pub year: u16,
    pub date_recorded: String,
    pub town: String,
    pub address: String,
    pub assessed_value: f64,
    pub sale_amount: f64,
    pub sales_ratio: f64,
    pub property_type: String,
    pub residential_type: Option<String>,
    pub location: Option<(f64, f64)>,
}
