use crate::property::Property;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Grid using integer lat/lon buckets to calculate average sales_ratio.
pub fn generate_heatmap(properties: &[Property], output_path: &str) {
    let mut grid: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    for p in properties.iter().filter(|p| p.sales_ratio > 0.0) {
        if let Some((lon, lat)) = p.location {
            // Scale and round coordinates to 0.01° (~1 km)
            let lon = (lon * 100.0).round() as i32;
            let lat = (lat * 100.0).round() as i32;
            grid.entry((lon, lat)).or_default().push(p.sales_ratio);
        }
    }

    let mut file = File::create(output_path).expect("Unable to create output file");
    writeln!(file, "lon,lat,avg_ratio").unwrap();

    for ((lon, lat), ratios) in grid {
        let avg = ratios.iter().sum::<f64>() / ratios.len() as f64;
        writeln!(file, "{:.4},{:.4},{:.3}", lon as f64 / 100.0, lat as f64 / 100.0, avg).unwrap();
    }

    println!("Heatmap CSV saved to {}", output_path);
}
