const CLUSTER_COUNT: usize = 16;

/// Generates sample JSON file containing coordinate pairs.
/// The pairs are clustered if requested to avoid a uniform polar distribution.
///
/// A JSON file and a binary .f64 files are generated as output.
///
/// The .f64 file contains the 64-bit floating point Haversine distance computation
/// result for each pair and the one final 64-bit float at the end for the mean.
pub fn generate_sample_json(
    distribution: PointsDistribution,
    random_seed: usize,
    coord_count: usize,
) {
    println!("Generate JSON");
    println!("{:?}", distribution);
    println!("{:?}", random_seed);
    println!("{:?}", coord_count);
}

#[derive(Debug)]
pub enum PointsDistribution {
    Uniform,
    Cluster,
}
