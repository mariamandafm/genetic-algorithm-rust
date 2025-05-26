use rand::prelude::*;
use std::collections::HashSet;
use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
//use std::thread;
//use std::time::Instant;
use std::time::Instant;

pub type Point = [f64; 2];
pub type Route = Vec<Point>;

// Distância entre dois pontos
fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2)).sqrt()   
}
// Distância total de uma rota
pub fn total_distance(route: &Route) -> f64 {
    let mut total = 0.0;
    for i in 1..route.len() {
        total += distance(&route[i - 1], &route[i]);
    }
    total
}

// Cria população inicial
fn create_population(rng: &mut ThreadRng, locations: &[Point], size: usize) -> Vec<Route> {
    let mut population = Vec::with_capacity(size);
    
    for _ in 0..size {
        let mut route = locations.to_vec();
        route.shuffle(rng); // Shuffle usando o RNG como fonte de aleatoriedade
        
        let mut full_route = Vec::with_capacity(route.len() + 2); // +2 para incluir o ponto inicial e final
        full_route.extend(route);
        
        population.push(full_route);
    }
    
    population
}

// Seleção por torneio
pub fn tournament_selection(rng: &mut ThreadRng, population: &[Route], k: usize) -> Route {
    let mut selected = Vec::with_capacity(k);
    // Seleciona k indivíduos aleatórios
    for _ in 0..k {
        let idx = rng.gen_range(0..population.len());
        selected.push(&population[idx]);
    }
    // Ordena os indivíduos selecionados pela função de fitness, pega o com melhor fitness
    // NOTE: Ao invés de total_distance, poderia usar fitness
    selected
        .into_iter()
        .min_by(|a, b| {
            total_distance(a)
                .partial_cmp(&total_distance(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
        .clone()
}


//Crossover entre dois pais
    pub fn crossover(rng: &mut ThreadRng, parent1: &Route, parent2: &Route) -> Route {
        let size = parent1.len();
        let start = rng.gen_range(0..size);
        let end = rng.gen_range(start..size);

        // Inicializa o filho com valores dummy
        let mut child = vec![[0.0, 0.0]; size]; // valor será sobrescrito
        let mut genes_in_child = HashSet::with_capacity(end - start + (size / 2));

        // Copia segmento do parent1 para o filho
        for i in start..=end {
            let gene = parent1[i];
            child[i] = gene;
            genes_in_child.insert(((gene[0].to_bits() as u128) << 64) | gene[1].to_bits() as u128);
        }

        // Preenche o restante com genes do parent2
        let mut parent2_index = 0;
        for i in 0..size {
            if parent2_index >= size {
                break;
            }
            if parent2_index == start {
                parent2_index = end + 1;
            }

            let gene = parent2[i];
            let key = ((gene[0].to_bits() as u128) << 64) | gene[1].to_bits() as u128; 
            if genes_in_child.insert(key) {
                child[i] = gene;
                parent2_index += 1;
            }
        }
        child
    }

// Mutação de uma rota
pub fn mutate(rng: &mut ThreadRng, route: &mut Route, mutation_rate: f64) {
    // Mutação simples: troca dois pontos aleatórios
    if rng.gen_bool(mutation_rate) {
        let i = rng.gen_range(1..route.len() - 1);
        let j = rng.gen_range(1..route.len() - 1);
        route.swap(i, j);
    }
    // Mutação de inversão: inverte uma parte da rota
    if rng.gen_bool(mutation_rate / 2.0) {
        let mut i = rng.gen_range(1..route.len() - 1);
        let mut j = rng.gen_range(1..route.len() - 1);
        
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        
        route[i..j].reverse();
    }
}

// Algoritmo genético principal
pub fn genetic_algorithm(
    rng: &mut ThreadRng,
    locations: &[Point],
    generations: usize,
    population_size: usize,
) -> Route {
    // println!("Criando população...");
    let mut population = create_population(rng, locations, population_size);
    // println!("População criada");
    let mut new_population = Vec::with_capacity(population_size);
    let mut children = Vec::with_capacity(population_size / 2);

    for _ in 0..generations {
        new_population.clear();
        children.clear();

        for _ in 0..population_size / 2 {
            new_population.push(tournament_selection(rng, &population, 2));
        }
        for _ in 0..(population_size - new_population.len()) {
            let parent1 = tournament_selection(rng, &population, 2);
            let parent2 = tournament_selection(rng, &population, 2);  
            let mut child = crossover(rng, &parent1, &parent2);
            mutate(rng, &mut child, 0.2);
            children.push(child);
        }
        new_population.extend(children.clone());
        population = new_population.clone();
    }
    // Retorna a melhor rota
    population
        .into_iter()
        .min_by(|a, b| {
            total_distance(a)
                .partial_cmp(&total_distance(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

// Salva a rota em um arquivo
pub fn save_distance_to_file(distance: f64, group_count: i32) -> std::io::Result<()> {
    let path = Path::new("distancias.txt");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "Grupo {}: Distância total: {:.6}\n", group_count, distance)?;
    
    Ok(())
}
