//! property.rs
//! This module defines the Property struct to represent a real estate transaction record, including geographic and financial fields used in data analysis.

/// Represents a single property record from the dataset.
/// This struct holds information such as:
///  -Location (town, address, lat/lon)
///  - Sale and assessed values
///  - Property classification details
/// Used throughout the program to carry data from CSV into analyses, graphs, and visualizations.
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    // A unique string ID representing the property (Serial Number column)
    pub serial_number: String,

    // The year associated with the listing (List Year column)
    pub year: u16,

    // The date the property was recorded as sold
    pub date_recorded: String,

    // The town where the property is located
    pub town: String,

    // The full street address of the property
    pub address: String,

    // The assessed value of the property (idk where this was determined but it is in the dataset and we will act as if it is correct)
    pub assessed_value: f64,

    // The actual amount the property was sold for
    pub sale_amount: f64,

    // The ratio of sale amount to assessed value (sale / assessed)
    pub sales_ratio: f64,

    // Type of the property (ie. Residential, Commercial)
    pub property_type: String,

    // Further residential classification (ie. Single Family, Condo, Duplex, etc.)
    pub residential_type: Option<String>,

    // Optional geolocation coordinates of the property (longitude, latitude) -> not in all records, However:
    // I created a seperate dataset that contained only entries with lat/lon coordinates which I will use for the purposes of this project
    // The code for the data spliting program is separate and not included in this project
    pub location: Option<(f64, f64)>,
}
