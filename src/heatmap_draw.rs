use crate::property::Property;
use plotters::prelude::*;
use std::collections::HashMap;

pub fn draw_heatmap(properties: &[Property], output_path: &str) {
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();

    for property in properties {
        if let Some((lon, lat)) = property.location {
            let lon_bin = (lon * 100.0).floor() as i32;
            let lat_bin = (lat * 100.0).floor() as i32;
            *grid.entry((lon_bin, lat_bin)).or_insert(0) += 1;
        }
    }

    // Set up canvas
    let root = BitMapBackend::new(output_path, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let min_lon = -73.8f64; // Connecticut west border approx
    let max_lon = -71.7f64; // Connecticut east border approx
    let min_lat = 40.9f64;  // Connecticut south border
    let max_lat = 42.1f64;  // Connecticut north border

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Connecticut Property Heatmap", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for ((lon_bin, lat_bin), count) in grid {
        let lon = lon_bin as f64 / 100.0;
        let lat = lat_bin as f64 / 100.0;

        let color = match count {
            0 => &WHITE,
            1..=5 => &BLUE,
            6..=15 => &GREEN,
            16..=50 => &YELLOW,
            51..=150 => &RED,
            _ => &BLACK,
        };

        chart.draw_series(std::iter::once(Circle::new((lon, lat), 2, color.filled()))).unwrap();
    }

    println!("Heatmap image saved to {}", output_path);
}
