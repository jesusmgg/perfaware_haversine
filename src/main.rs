mod data;

use std::env;

use data::json::{generate_sample_json, PointsDistribution};

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len < 2 {
        print_help();
        return Ok(());
    }

    let mut selected_operation: Option<ProgramOperation> = None;
    let mut selected_arguments: Vec<&str> = Vec::new();
    let mut selected_options: Vec<ProgramOption> = Vec::new();

    for arg in &args[1..args_len] {
        match arg.as_str() {
            "generate-data" => selected_operation = Some(ProgramOperation::GenerateData),
            "--greet" => selected_options.push(ProgramOption::Greet),
            program_arg => selected_arguments.push(program_arg),
        }
    }

    return match selected_operation {
        Some(operation) => match operation {
            ProgramOperation::GenerateData => start_generate_data(&selected_arguments),
        },
        None => {
            print_help();
            Ok(())
        }
    };
}

fn print_help() {
    println!("Usage: haversine OPERATION ARGUMENTS");
    println!("\nOperations:");
    println!("  generate-data [uniform/cluster] [random seed] [number of coord pairs]");
    println!();
}

fn start_generate_data(selected_arguments: &Vec<&str>) -> Result<(), ()> {
    if selected_arguments.len() < 3 {
        print_help();
        return Ok(());
    }

    let distribution = match selected_arguments[0] {
        "uniform" => PointsDistribution::Uniform,
        "cluster" => PointsDistribution::Cluster,
        _ => {
            print_help();
            return Ok(());
        }
    };

    let seed = match selected_arguments[1].parse::<usize>() {
        Ok(seed) => seed,
        _ => {
            print_help();
            return Ok(());
        }
    };

    let coord_count = match selected_arguments[2].parse::<usize>() {
        Ok(count) => count,
        _ => {
            print_help();
            return Ok(());
        }
    };

    generate_sample_json(distribution, seed, coord_count);

    Ok(())
}

enum ProgramOperation {
    GenerateData,
}

enum ProgramOption {
    Greet,
}
