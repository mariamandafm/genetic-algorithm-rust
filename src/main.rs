use std::time::Instant;
use genetic_algorithm_rust::{GeneticAlgorithmPathFinding, Point, process_data}; 

fn main() {
    // Exemplo de uso
    let mut ga = GeneticAlgorithmPathFinding::new();
    
    // let locations = vec![
    //     [1.0, 2.0],
    //     [3.0, 4.0],
    //     [5.0, 6.0],
    //     [7.0, 8.0],
    //     [9.0, 10.0],
    // ];
    //let start_process_data = Instant::now();
    let locations = process_data::get_coordinates_from_csv("pedidos_entrega.csv", "São Paulo").unwrap();
    //let duration_process_data = start_process_data.elapsed();
    //println!("Tempo de execução processar dados: {:.2?}", duration_process_data);
    let start = [0.0, 0.0];

    
    //println!("Iniciando algoritmo genético...");
    //let start_process_alg = Instant::now();
    let best_route = ga.genetic_algorithm(&locations, &start, 10, 10);
    //let duration_process_alg = start_process_alg.elapsed();
    //println!("Tempo de execução algoritmo: {:.2?}", duration_process_alg);
    println!("Processamento finalizado");
    
    println!("Distância total da melhor rota: {}", ga.total_distance(&best_route));
    
    if let Err(e) = ga.save_route_to_file(&best_route, "melhor_rota.txt") {
        eprintln!("Erro ao salvar a rota: {}", e);
    }
}