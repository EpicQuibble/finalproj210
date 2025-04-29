use crate::property::Property;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Build and export a simple density heatmap by location.
/// Groups into 0.01 degree grid squares.
pub fn generate_heatmap(properties: &[Property], output_file: &str) {
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();

    for property in properties {
        if let Some((lon, lat)) = property.location {
            let lon_bin = (lon * 100.0).floor() as i32;
            let lat_bin = (lat * 100.0).floor() as i32;
            *grid.entry((lon_bin, lat_bin)).or_insert(0) += 1;
        }
    }

    let mut file = File::create(output_file).expect("Cannot create heatmap file");

    writeln!(file, "LongitudeBin,LatitudeBin,Count").unwrap();
    for ((lon_bin, lat_bin), count) in grid {
        writeln!(file, "{},{},{}", lon_bin as f64 / 100.0, lat_bin as f64 / 100.0, count).unwrap();
    }

    println!("Heatmap saved to {}", output_file);
}
