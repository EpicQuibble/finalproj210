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

fn main() {
    let start = Instant::now();

    let properties = parse_csv("cords_split_only.csv");
    println!("Loaded {} properties.", properties.len());

    print_summary_statistics(&properties);
    generate_heatmap(&properties, "ct_heatmap.csv");
    draw_heatmap(&properties, "ct_heatmap.png");

    find_rising_towns(&properties);
    find_flipper_properties(&properties);

    println!("Program completed in {:.2?}", start.elapsed());
}
