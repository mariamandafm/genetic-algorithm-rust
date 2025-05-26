use criterion::{criterion_group, criterion_main, Criterion};
use genetic_algorithm_rust::genetic_algorithm::{genetic_algorithm, crossover, total_distance, mutate, tournament_selection, Route};
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;
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

// pub fn generate_route(file_path: &str, group_index: usize) -> Result<Route, Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let reader = BufReader::new(file);

//     let mut current_group = Vec::new();
//     let mut current_index = 0;

//     for line in reader.lines() {
//         let line = line?;
//         if line.trim() == "-" {
//             if current_index == group_index {
//                 break;
//             }
//             current_group.clear();
//             current_index += 1;
//             continue;
//         }

//         let parts: Vec<&str> = line.split(';').collect();
//         if parts.len() != 2 {
//             continue;
//         }

//         let latitude_str = parts[0].trim().replace(",", ".");
//         let longitude_str = parts[1].trim().replace(",", ".");

//         if let (Ok(latitude), Ok(longitude)) = (
//             latitude_str.parse::<f64>(),
//             longitude_str.parse::<f64>(),
//         ) {
//             if current_index == group_index {
//                 current_group.push([latitude, longitude]);
//             }
//         }
//     }

//     Ok(current_group)
// }
fn benchmark_crossover(c: &mut Criterion) {
    let parent1 = generate_route(50_000);
    let parent2 = generate_route(50_000);
    let mut rng = rand::thread_rng();
    c.bench_function("crossover_50k", |b| {
        b.iter(|| {
            crossover(&mut rng, &parent1, &parent2);
        })
    });
}

fn benchmark_genetic_algorithm(c: &mut Criterion) {
    let locations = generate_route(50_000);
    let mut rng = rand::thread_rng();
    c.bench_function("genetic_algorithm_50k", |b| {
        b.iter(|| {
            genetic_algorithm(&mut rng, &locations, 10, 10);
        })
    });
}

fn benchmark_distance(c: &mut Criterion) {
    let route = generate_route(50_000);
    c.bench_function("total_distance_50k", |b| {
        b.iter(|| {
            total_distance(&route);
        })
    });
}

fn bechmark_mutate(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut route = generate_route(50_000);
    c.bench_function("mutate_50k", |b| {
        b.iter(|| {
            mutate(&mut rng, &mut route, 0.2);
        })
    });
}

fn benchmark_tournament_selection(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let population = (0..10).map(|_| generate_route(50_000)).collect::<Vec<_>>();
    c.bench_function("tournament_selection_50k", |b| {
        b.iter(|| {
            tournament_selection(&mut rng, &population, 2);
        })
    });
}

criterion_group!(benches, benchmark_genetic_algorithm, benchmark_crossover, benchmark_distance, bechmark_mutate, benchmark_tournament_selection);
//criterion_group!(benches, benchmark_crossover);
criterion_main!(benches);
