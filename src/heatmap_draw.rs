//! heatmap_draw.rs
//! This module generates a visual heatmap of property density using Plotters, with a color legend

use crate::property::Property;
use plotters::prelude::*;
use std::collections::HashMap;

/// Draws a PNG heatmap showing property counts by spatial grid cell
/// I want to thank Sam because this was his idea when I asked him in office hours what to do with the data
///
/// Inputs -> properties list of property records with geographic coordinates, output_path path to output PNG file
/// Outputs -> saves a rendered heatmap image with a bottom legend
///
/// High-Level Logic:
///  - Bin properties into 0.01-degree grid cells
///  - Count properties per cell
///  - Plot circles colored by count tier
///  - Add labeled color legend at the bottom
pub fn draw_heatmap(properties: &[Property], output_path: &str) {
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();

    for property in properties {
        if let Some((lon, lat)) = property.location {

            // Convert to integer bins by multiplying coordinates
            let lon_bin = (lon * 100.0).floor() as i32;
            let lat_bin = (lat * 100.0).floor() as i32;
            *grid.entry((lon_bin, lat_bin)).or_insert(0) += 1;
        }
    }

    // Create a 1024×1100 canvas: map on top, legend on bottom
    let root = BitMapBackend::new(output_path, (1024, 1100)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Set map bounds to connecticuts approximate coordinates
    let (chart_area, legend_area) = root.split_vertically(1024);

    let min_lon = -73.8;
    let max_lon = -71.7;
    let min_lat = 40.9;
    let max_lat = 42.1;

    let mut chart = ChartBuilder::on(&chart_area)
        .margin(10)
        .caption("Connecticut Property Density Heatmap", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for ((lon_bin, lat_bin), count) in &grid {

        // Convert bin index back to coordinates 
        let lon = *lon_bin as f64 / 100.0;
        let lat = *lat_bin as f64 / 100.0;

        // Color-coding by count thresholds
        let color = match count {
            1..=5 => RGBColor(173, 216, 230),       // Light Blue
            6..=15 => RGBColor(144, 238, 144),      // Light Green
            16..=50 => RGBColor(255, 255, 0),       // Yellow
            51..=150 => RGBColor(255, 165, 0),      // Orange
            _ => RED,                               // 150 += RED
        };

        chart.draw_series(std::iter::once(Circle::new((lon, lat), 2, color.filled()))).unwrap();
    }

// Legend entries with optional color
let labels = vec![
    ("Sales Count: # of homes ", None),  // Label only, no box
    ("1-5", Some(RGBColor(173, 216, 230))),
    ("6-15", Some(RGBColor(144, 238, 144))),
    ("16-50", Some(RGBColor(255, 255, 0))),
    ("51-150", Some(RGBColor(255, 165, 0))),
    ("150+", Some(RED)),
];

let mut x = 20;
for (label, color_opt) in labels {
    match color_opt {
        None => {
                // Draw plain text label
            legend_area
                .draw_text(label, &("sans-serif", 22).into_text_style(&legend_area), (x, 12))
                .unwrap();
            x += 220; //  More space after the label
        }
        Some(color) => {
            // Draw box and label together
            legend_area
                .draw(&Rectangle::new([(x, 10), (x + 20, 30)], color.filled()))
                .unwrap();
            legend_area
                .draw_text(label, &("sans-serif", 20).into_text_style(&legend_area), (x + 30, 12))
                .unwrap();
            x += 130; // Spacing between boxes
        }
    }
}

    println!("Heatmap image saved to {}", output_path);
}
