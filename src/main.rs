use std::time::Instant;
use genetic_algorithm_rust::process_coordinates::{calculate_best_route}; 
use genetic_algorithm_rust::process_coordinates_v2::calculate_best_route_v2;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use genetic_algorithm_rust::{genetic_algorithm::{genetic_algorithm, total_distance, save_distance_to_file, Route}};
type Point = [f64; 2];
use rand::{Rng, thread_rng};

fn main() {

    let start_process_data = Instant::now();

    let locations = calculate_best_route("coordenadas_1000_50.txt");
    let duration_process_data = start_process_data.elapsed();

    println!("Tempo de execução processar dados: {:.2?}", duration_process_data);
    println!("Processamento finalizado");
}