use criterion::{black_box, criterion_group, criterion_main, Criterion};
use genetic_algorithm_rust::{GeneticAlgorithmPathFinding, process_data}; // substitua pelo nome do seu crate

fn bench_genetic_algorithm() -> f64 {
    let locations = process_data::get_coordinates_from_csv("pedidos_entrega.csv", "Natal").unwrap();
    let start = [0.0, 0.0];
    let mut ga = GeneticAlgorithmPathFinding::new();
    let best_route = ga.genetic_algorithm(&locations, &start, 10, 10);
    ga.total_distance(&best_route)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("genetic_algorithm");

    // Definir o número de amostras
    group
        .sample_size(10)  // Aqui você escolhe o número de amostras
        .measurement_time(std::time::Duration::from_secs(60)) // Ajuste conforme necessário
        .bench_function("run_genetic_algorithm", |b| {
            b.iter(|| {
                // Usando black_box para impedir otimizações do compilador
                black_box(bench_genetic_algorithm());
            });
        });

    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);