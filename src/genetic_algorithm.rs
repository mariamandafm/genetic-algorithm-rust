use rand::prelude::*;
use std::collections::HashSet;
use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
//use std::thread;
//use std::time::Instant;

pub type Point = [f64; 2];
pub type Route = Vec<Point>;

pub struct GeneticAlgorithmPathFinding {
    rng: ThreadRng,
}

impl GeneticAlgorithmPathFinding {
    pub fn new() -> Self {
        GeneticAlgorithmPathFinding {
            rng: rand::thread_rng(),
        }
    }

    // Distância entre dois pontos
    fn distance(&self, p1: &Point, p2: &Point) -> f64 {
        ((p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2)).sqrt()
    }

    // Distância total de uma rota
    pub fn total_distance(&self, route: &Route) -> f64 {
        route.windows(2).map(|w| self.distance(&w[0], &w[1])).sum()
    }

    // Cria população inicial
    fn create_population(&mut self, locations: &[Point], size: usize) -> Vec<Route> {
        let mut population = Vec::with_capacity(size);
        
        for _ in 0..size {
            let mut route = locations.to_vec();
            route.shuffle(&mut self.rng); // Shuffle usando o RNG como fonte de aleatoriedade
            
            let mut full_route = Vec::with_capacity(route.len() + 2); // +2 para incluir o ponto inicial e final
            full_route.extend(route);
            
            population.push(full_route);
        }
        
        population
    }

    // Seleção por torneio
    fn tournament_selection(&mut self, population: &[Route], k: usize) -> Route {
        let mut selected = Vec::with_capacity(k);
        // Seleciona k indivíduos aleatórios
        for _ in 0..k {
            let idx = self.rng.gen_range(0..population.len());
            selected.push(&population[idx]);
        }
        // Ordena os indivíduos selecionados pela função de fitness, pega o com melhor fitness
        // NOTE: Ao invés de total_distance, poderia usar fitness
        selected
            .into_iter()
            .min_by(|a, b| {
                self.total_distance(a)
                    .partial_cmp(&self.total_distance(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone()
    }

    //Crossover entre dois pais
    pub fn crossover(&mut self, parent1: &Route, parent2: &Route) -> Route {
        let size = parent1.len();
        let start = self.rng.gen_range(0..size);
        let end = self.rng.gen_range(start..size);

        // Inicializa o filho com valores dummy
        let mut child = vec![[0.0, 0.0]; size]; // valor será sobrescrito
        let mut filled = vec![false; size]; // para marcar se a posição já foi preenchida
        let mut genes_in_child = HashSet::with_capacity(end - start + (size / 2));

        // Copia segmento do parent1 para o filho
        for i in start..=end {
            let gene = parent1[i];
            child[i] = gene;
            genes_in_child.insert((gene[0].to_bits(), gene[1].to_bits()));
            filled[i] = true;
        }

        // Preenche o restante com genes do parent2
        let mut parent2_index = 0;
        for i in 0..size {
            if filled[i] {
                continue;
            }

            while parent2_index < size {
                let gene = parent2[parent2_index];
                parent2_index += 1;
                let key = (gene[0].to_bits(), gene[1].to_bits());
                if !genes_in_child.contains(&key) {
                    child[i] = gene;
                    genes_in_child.insert(key);
                    break;
                }
            }
        }

        child
    }

    // Mutação de uma rota
    fn mutate(&mut self, route: &mut Route, mutation_rate: f64) {
        // Mutação simples: troca dois pontos aleatórios
        if self.rng.gen_bool(mutation_rate) {
            let i = self.rng.gen_range(1..route.len() - 1);
            let j = self.rng.gen_range(1..route.len() - 1);
            route.swap(i, j);
        }
        // Mutação de inversão: inverte uma parte da rota
        if self.rng.gen_bool(mutation_rate / 2.0) {
            let mut i = self.rng.gen_range(1..route.len() - 1);
            let mut j = self.rng.gen_range(1..route.len() - 1);
            
            if i > j {
                std::mem::swap(&mut i, &mut j);
            }
            
            route[i..j].reverse();
        }
    }

    // Algoritmo genético principal
    pub fn genetic_algorithm(
        &mut self,
        locations: &[Point],
        generations: usize,
        population_size: usize,
    ) -> Route {
        // println!("Criando população...");
        let mut population = self.create_population(locations, population_size);
        // println!("População criada");
        
        for generation in 0..generations {
            // println!("Geração {}", generation);
            let mut new_population = Vec::with_capacity(population_size);
            
            // Seleção por torneio para metade da população
            for _ in 0..population_size / 2 {
                new_population.push(self.tournament_selection(&population, 2));
            }
            
            // Crossover e mutação para o restante
            let mut children = Vec::with_capacity(population_size - new_population.len());
            for _ in 0..(population_size - new_population.len()) {
                let parent1 = self.tournament_selection(&population, 2); // Seleciona um parente por torneio
                let parent2 = self.tournament_selection(&population, 2); // Seleciona outro parente por torneio
                let mut child = self.crossover(&parent1, &parent2); // Faz o crossover
                self.mutate(&mut child, 0.2);
        
                children.push(child);
            }
            
            new_population.extend(children);
            population = new_population;
            
            // println!("Geração {} completada", generation);
        }
        
        // Retorna a melhor rota
        population
            .into_iter()
            .min_by(|a, b| {
                self.total_distance(a)
                    .partial_cmp(&self.total_distance(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }

    // Salva a rota em um arquivo
    pub fn save_distance_to_file(&self, distance: f64, group_count: i32) -> std::io::Result<()> {
        let path = Path::new("distancias.txt");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "Grupo {}: Distância total: {:.6}\n", group_count, distance)?;
        
        Ok(())
    }
}