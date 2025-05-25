use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::genetic_algorithm::{genetic_algorithm, total_distance, save_distance_to_file, Route};
type Point = [f64; 2];
use rand::{Rng, thread_rng};
use std::time::Instant;

fn generate_route(size: usize) -> Route {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            let lat = rng.gen_range(-90.0..90.0);
            let lon = rng.gen_range(-180.0..180.0);
            [lat, lon]
        })
        .collect()
}

pub fn get_coordinates_from_csv(file_path: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut rng = thread_rng();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut current_group = Vec::new();

    let mut group_count = 0;
    for line in reader.lines() {

        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();

        if line == "-"{
            group_count += 1;
            println!("{}", group_count);
            // let test_group = generate_route(50_000);
             let start_process_data = Instant::now();
            let best_route = genetic_algorithm(&mut rng, &current_group, 10, 10);
             let duration_process_data = start_process_data.elapsed();
             println!("Tempo de execução processar dados: {:.2?}", duration_process_data);

            let distance = total_distance(&best_route);
            save_distance_to_file(distance, group_count)?;
            println!("Distância total: {}", total_distance(&current_group));
            current_group.clear();
            continue;
        }

        let latitude_str = format!("{}", parts[0].trim()).replace(",", ".");
        let longitude_str = format!("{}", parts[1].trim()).replace(",", ".");
        if let (Ok(latitude), Ok(longitude)) = (
            latitude_str.parse::<f64>(),
            longitude_str.parse::<f64>(),
        ) {
            current_group.push([latitude, longitude]);
        } else {
            //eprintln!("Erro ao converter coordenadas: {:?}", parts);
        }
    }

    if !current_group.is_empty() {
        group_count += 1;
        println!("{}, {}", group_count, current_group.len());
        let best_route = genetic_algorithm(&mut rng, &current_group, 10, 10);
        let distance = total_distance(&best_route);
        save_distance_to_file(distance, group_count)?;
        current_group.clear();
    }

    Ok(current_group)
}
