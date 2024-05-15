use std::{
    fs::File,
    io::{BufWriter, Write},
};

use quad_rand as qrand;
use serde::Serialize;

const CLUSTER_COUNT: usize = 16;
const EARTH_RADIUS: f64 = 6372.8;

/// Generates sample JSON file containing coordinate pairs.
/// The pairs are clustered if requested to avoid a uniform polar distribution.
///
/// A JSON file and a binary .f64 files are generated as output.
///
/// The .f64 file contains the 64-bit floating point Haversine distance computation
/// result for each pair and the one final 64-bit float at the end for the mean.
pub fn generate_sample_json(distribution: PointsDistribution, random_seed: u64, pair_count: usize) {
    println!("Generating sample data...");
    println!("Method: {:?}", distribution);
    println!("Random seed: {}", random_seed);
    println!("Pair count: {}", pair_count);

    qrand::srand(random_seed);

    // Generate clusters
    let mut clusters = Vec::with_capacity(CLUSTER_COUNT);
    if let PointsDistribution::Cluster = distribution {
        for _ in 0..CLUSTER_COUNT {
            let x0 = qrand::gen_range(-180.0, 180.0);
            let y0 = qrand::gen_range(-90.0, 90.0);
            let x1 = qrand::gen_range(-180.0, 180.0);
            let y1 = qrand::gen_range(-90.0, 90.0);

            let cluster = (
                f64::min(x0, x1),
                f64::min(y0, y1),
                f64::max(x0, x1),
                f64::max(y0, y1),
            );

            clusters.push(cluster);
        }
    }

    // Generate pairs
    let mut pairs_container = PairsContainer {
        pairs: Vec::with_capacity(pair_count),
    };
    let mut haversines: Vec<f64> = Vec::with_capacity(pair_count);
    let mut sum: f64 = 0.0;
    let mean: f64;

    for i in 0..pair_count {
        let cluster = match distribution {
            PointsDistribution::Uniform => (-180.0, -90.0, 180.0, 90.0),
            PointsDistribution::Cluster => clusters[i % CLUSTER_COUNT],
        };

        let min_x = cluster.0;
        let min_y = cluster.1;
        let max_x = cluster.2;
        let max_y = cluster.3;

        let x0 = qrand::gen_range(min_x, max_x);
        let y0 = qrand::gen_range(min_y, max_y);
        let x1 = qrand::gen_range(min_x, max_x);
        let y1 = qrand::gen_range(min_y, max_y);
        let haversine = calculate_haversine(x0, y0, x1, y1, EARTH_RADIUS);

        sum += haversine;

        pairs_container.pairs.push(Pair { x0, y0, x1, y1 });
        haversines.push(haversine);
    }

    mean = sum / pair_count as f64;
    println!("Expected sum (mean): {}", mean);

    // Write output files
    println!("Writing to 'data.json'...");
    let json_file = File::create("data.json").unwrap();
    let mut writer = BufWriter::new(json_file);
    serde_json::to_writer(&mut writer, &pairs_container).unwrap();
    writer.flush().unwrap();
    println!("Done");
}

#[derive(Debug)]
pub enum PointsDistribution {
    Uniform,
    Cluster,
}

// TODO: move code to correct place
/// Calculate Harversine distance using provided formula. Parameter coordinates are in degrees.
fn calculate_haversine(x0: f64, y0: f64, x1: f64, y1: f64, earth_radius: f64) -> f64 {
    let mut lat1 = y0;
    let mut lat2 = y1;
    let lon1 = x0;
    let lon2 = x1;

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    lat1 = lat1.to_radians();
    lat2 = lat2.to_radians();

    let a = ((d_lat / 2.0).sin()).powi(2) + lat1.cos() * lat2.cos() * ((d_lon / 2.0).sin()).powi(2);
    let c = 2.0 * (a.sqrt()).asin();

    let result = earth_radius * c;

    result
}

#[derive(Serialize)]
struct PairsContainer {
    pub pairs: Vec<Pair>,
}

#[derive(Serialize)]
struct Pair {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}
