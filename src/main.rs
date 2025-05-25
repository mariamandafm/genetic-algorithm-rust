use std::time::Instant;
use genetic_algorithm_rust::process_coordinates::{get_coordinates_from_csv}; 

fn main() {
    let start_process_data = Instant::now();

    let locations = get_coordinates_from_csv("coordenadas_200_50.txt").unwrap();
    let duration_process_data = start_process_data.elapsed();

    println!("Tempo de execução processar dados: {:.2?}", duration_process_data);
    println!("Coordenadas encontradas: {}", locations.len());
    println!("Processamento finalizado");
}