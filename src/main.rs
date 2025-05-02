//! main.rs
//! Entry point for the program. It loads data, performs analysis, generates a heatmap, and reports insights.

mod parser;
mod property;
mod heatmap;
mod heatmap_draw;
mod insights;

use parser::parse_csv;
use heatmap::{generate_heatmap};
use heatmap_draw::draw_heatmap;
use insights::{print_summary_statistics, find_rising_towns, find_flipper_properties};
use std::time::Instant;



///  What it does:
///  - Loads CSV data into memory
///  - Computes summary statistics (mean, median etc)
///  - Generates a CSV heatmap and image heatmap
///  - Finds towns with rising sale prices
///  - Identifies undervalued flippable properties 
///
///  Inputs -> None directly, the chosen CSV file  
///
///  Outputs -> Printed summary statistics
///  - Heatmap files written to disk (ct_heatmap.csv, and ct_heatmap.png)
///  - Console output of rising towns and flipper properties
fn main() {
    let start = Instant::now(); // Measure program runtime

    // Load properties from CSV file
    let properties = parse_csv("shrunk_split_data.csv"); // Put whatever csv file you want here
    println!("Loaded {} properties.", properties.len());

    // Print high-level summary stats (mean, median, std dev)
    print_summary_statistics(&properties);

    // Generate a grid-based CSV heatmap of average sales ratios
    generate_heatmap(&properties, "ct_heatmap.csv");

    // Generate a PNG heatmap image of property sale density
    draw_heatmap(&properties, "ct_heatmap.png");

    // Identify towns with high recent price growth based on median change
    find_rising_towns(&properties);
    
    // Identify low-price homes that resemble high-end ones in wealthy areas
    find_flipper_properties(&properties);

    // Display total execution time
    println!("Program completed in {:.2?}", start.elapsed());
}
