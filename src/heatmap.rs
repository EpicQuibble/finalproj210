//! heatmap.rs
//! This module generates a grid-based heatmap CSV showing the average sales_ratio, per spatial cell using latitude and longitude coordinates

use crate::property::Property;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Generates a heatmap by calculating the average sales_ratio for each coordinate bin, and writes the result to a CSV file
///
/// Inputs -> properties: location data, output_path and the path to the CSV file to be created
/// Outputs -> A CSV file with columns: lon, lat, avg_ratio
///
/// High-Level Logic:
///  - Filters valid properties with a positive sales ratio
///  - Bins each property's location into a discrete (lon, lat) grid
///  - Stores all ratios in each bin
///  - Averages each bin's values and writes them to the file
/// 
pub fn generate_heatmap(properties: &[Property], output_path: &str) {
    let mut grid: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    for p in properties.iter().filter(|p| p.sales_ratio > 0.0) {
        if let Some((lon, lat)) = p.location {
            // Scale and round coordinates to form grid bins
            let lon = (lon * 100.0).round() as i32;
            let lat = (lat * 100.0).round() as i32;

            // Group sales_ratios into corresponding (lon, lat) grid cell
            grid.entry((lon, lat)).or_default().push(p.sales_ratio);
        }
    }

    // Create or overwrite the output file
    let mut file = File::create(output_path).expect("Unable to create output file");

    // Write a CSV header
    writeln!(file, "lon,lat,avg_ratio").unwrap();

    for ((lon, lat), ratios) in grid {
        // Compute average ratio for each grid cell
        let avg = ratios.iter().sum::<f64>() / ratios.len() as f64;

        // Convert coordinates back to float format for output
        writeln!(file, "{:.4},{:.4},{:.3}", lon as f64 / 100.0, lat as f64 / 100.0, avg).unwrap();
    }

    println!("Heatmap CSV saved to {}", output_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::Property;
    use std::fs;

    #[test]
    fn test_generate_heatmap_content() {
        let props = vec![
            Property {
                sales_ratio: 0.5,
                location: Some((-72.55, 41.23)),
                ..Default::default()
            },
            Property {
                sales_ratio: 1.5,
                location: Some((-72.55, 41.23)),
                ..Default::default()
            },
        ];

        let output = "test_heatmap.csv";
        generate_heatmap(&props, output);

        let file_contents = fs::read_to_string(output).unwrap();

        // Make sure the header is present
        assert!(file_contents.contains("lon,lat,avg_ratio"));

        // Make sure the average is calculated correctly: (0.5 + 1.5) / 2 = 1.0
        assert!(file_contents.contains("41.23"));
        assert!(file_contents.contains("1.000"));

       fs::remove_file(output).unwrap();
    }
}
    