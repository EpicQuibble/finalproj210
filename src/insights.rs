//! insights.rs
//! This module provides analytical functions for summarizing statistics and detecting flippable undervalued properties

use crate::property::Property;
use std::collections::HashMap;

/// Prints basic summary statistics (mean, median, mode, standard deviation)
/// for the sale amounts of all valid properties.
///
/// Inputs -> properties: [Property]  *I didnt know you could do that with the hover over thing in brackets*
///
/// Outputs: -> Printed summary statistics to stdout
///
/// High-Level Logic:
///  - Filters out invalid sale amounts
///  - Computes summary metrics using iterator operations
pub fn print_summary_statistics(properties: &[Property]) {
    let mut sales: Vec<f64> = properties.iter().map(|p| p.sale_amount).filter(|v| *v > 0.0).collect();
    sales.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = sales.iter().sum::<f64>() / sales.len() as f64;
    let median = sales[sales.len() / 2];
    let mode = find_mode(&sales);
    let std_dev = (sales.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / sales.len() as f64).sqrt();

    println!("  Summary Statistics:");
    println!("  Mean Sale Amount: ${:.2}", mean);
    println!("  Median Sale Amount: ${:.2}", median);
    println!("  Mode Sale Amount: ${:.2}", mode);
    println!("  Std Dev: ${:.2}", std_dev);
}

/// Computes the mode (most common value) in a list of sales prices
///
/// Inputs:
/// sales, This is a slice of sale amounts
/// 
/// Outputs -> Most frequent rounded value in the dataset
///
/// High-Level Logic:
///  - Rounds sales to nearest thousand to reduce noise
///  - Counts frequency of each value
///  - Returns the most frequent one
/// 
fn find_mode(sales: &[f64]) -> f64 {
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &s in sales {
        // Round to nearest 1000 to simplify mode grouping
        let rounded = ((s / 1000.0).round() * 1000.0) as i64;
        *freq.entry(rounded).or_insert(0) += 1;
    }

    // Find the value with the highest count
    freq.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val as f64)
        .unwrap_or(0.0)
}

/// Identifies towns with rising home prices by analyzing the median price trend across years.
///
/// Inputs -> properties [Property]
///
/// Outputs -> Prints top 5 towns with the highest percentage growth based on median price
///
/// High-Level Logic:
///  - Organizes sales by town and year
///  - Computes yearly medians for each town
///  - Calculates percent change from first to last year
pub fn find_rising_towns(properties: &[Property]) {
    let mut by_town_year: HashMap<(&String, u16), Vec<f64>> = HashMap::new();

    for p in properties.iter().filter(|p| p.sale_amount > 0.0) {
        // Group sale amounts by (town, year) for calcukating median 
        by_town_year.entry((&p.town, p.year)).or_default().push(p.sale_amount);
    }

    let mut growth_data: Vec<(&String, f64)> = Vec::new();

    let years = vec![2021, 2022, 2023]; // Analyze price trends across these years

    for town in properties.iter().map(|p| &p.town).collect::<std::collections::HashSet<_>>() {
        let mut medians = vec![];
        for &year in &years {
            // Compute median for each year
            if let Some(sales) = by_town_year.get(&(town, year)) {
                let mut s = sales.clone();
                s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                medians.push(s[s.len() / 2]);
            }
        }
        if medians.len() >= 2 {
            // Calculate the growth trend from first to last year
            let trend = (medians[medians.len() - 1] - medians[0]) / medians[0];
            growth_data.push((town, trend));
        }
    }

    // Sort towns by growth trend (descending)
    growth_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Top 5 Towns Predicted to Rise (Median Sale Growth):");
    for (town, growth) in growth_data.iter().take(5) {
        println!("  {}: {:.1}% median price increase", town, growth * 100.0);
    }
}

/// Identifies undervalued properties that are similar to high-end homes in the same town/type.
///
/// Inputs -> properties [Property]
///
/// Outputs -> prints up to 5 flippable property opportunities
///
/// High-Level Logic:
///  - First finds expensive, well-valued "high-end" homes
///  - Then searches for cheap homes with similar characteristics in the same towns
///  - Filters out junk entries and duplicates
///  - Prints the top 5 candidates for flipper properties
pub fn find_flipper_properties(properties: &[Property]) {
    let high_end = properties.iter()
        .filter(|p: &&Property| p.sale_amount > 500_000.0 && p.sales_ratio > 1.0 && p.assessed_value > 100_000.0)
        .collect::<Vec<_>>();

    let mut candidates = Vec::new();
    let mut seen_addresses = std::collections::HashSet::new();

    for cheap in properties.iter().filter(|p| {
        // Basic filters for flippable homes: reasonable price, valid assessment, below avg ratio
        p.sale_amount >= 50_000.0 &&
        p.sale_amount <= 300_000.0 &&
        p.sales_ratio > 0.1 &&
        p.sales_ratio < 0.85 &&
        p.assessed_value >= 50_000.0
    }) {
        for rich in &high_end {
            // Compare by town and property type to detect similarities
            if cheap.property_type == rich.property_type && cheap.town == rich.town {
                // Avoid duplicate candidates by address
                if seen_addresses.insert(&cheap.address) {
                    candidates.push(cheap);
                }
                break;
            }
        }
    }

    println!("Flipper Property Candidates (Low Price, High-End Traits):");
    for prop in candidates.iter().take(5) {
        println!(
            "  {} in {}: ${:.0} (Assessed ${:.0}, Ratio {:.2})",
            prop.address,
            prop.town,
            prop.sale_amount,
            prop.assessed_value,
            prop.sales_ratio
        );
    }
}
