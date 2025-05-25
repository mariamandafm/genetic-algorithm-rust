use criterion::{criterion_group, criterion_main, Criterion};
use genetic_algorithm_rust::genetic_algorithm::{GeneticAlgorithmPathFinding, Route};
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
// fn generate_route(size: usize) -> Route {
//     let mut rng = rand::thread_rng();
//     (0..size)
//         .map(|_| {
//             let lat = rng.gen_range(-90.0..90.0);
//             let lon = rng.gen_range(-180.0..180.0);
//             [lat, lon]
//         })
//         .collect()
// }

pub fn generate_route(file_path: &str, group_index: usize) -> Result<Route, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut current_group = Vec::new();
    let mut current_index = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim() == "-" {
            if current_index == group_index {
                break;
            }
            current_group.clear();
            current_index += 1;
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            continue;
        }

        let latitude_str = parts[0].trim().replace(",", ".");
        let longitude_str = parts[1].trim().replace(",", ".");

        if let (Ok(latitude), Ok(longitude)) = (
            latitude_str.parse::<f64>(),
            longitude_str.parse::<f64>(),
        ) {
            if current_index == group_index {
                current_group.push([latitude, longitude]);
            }
        }
    }

    Ok(current_group)
}
// fn benchmark_crossover(c: &mut Criterion) {
//     let mut algo = GeneticAlgorithmPathFinding::new();
//     let parent1 = generate_route(50_000);
//     let parent2 = generate_route(50_000);
//     c.bench_function("crossover_50k", |b| {
//         b.iter(|| {
//             algo.crossover(&parent1, &parent2);
//         })
//     });
// }

fn benchmark_genetic_algorithm(c: &mut Criterion) {
    let mut algo = GeneticAlgorithmPathFinding::new();
    let locations = generate_route("coordenadas_200_50.txt", 0).unwrap();
    c.bench_function("genetic_algorithm_50k", |b| {
        b.iter(|| {
            algo.genetic_algorithm(&locations, 10, 10);
        })
    });
}

criterion_group!(benches, benchmark_genetic_algorithm);
criterion_main!(benches);
