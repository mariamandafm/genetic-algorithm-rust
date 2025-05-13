use rand::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::panic::Location;
use std::path::Path;
pub mod process_data;
use std::thread;
use std::time::Instant;

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
    fn create_population(&mut self, locations: &[Point], start: &Point, size: usize) -> Vec<Route> {
        let mut population = Vec::with_capacity(size);
        
        for _ in 0..size {
            let mut route = locations.to_vec();
            route.shuffle(&mut self.rng); // Shuffle usando o RNG como fonte de aleatoriedade
            
            let mut full_route = Vec::with_capacity(route.len() + 2); // +2 para incluir o ponto inicial e final
            full_route.push(*start);
            full_route.extend(route);
            full_route.push(*start);
            
            population.push(full_route);
        }
        
        population
    }

    // Função de fitness (quanto maior, melhor)
    fn fitness(&self, route: &Route) -> f64 {
        1.0 / self.total_distance(route)
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
    fn crossover(&mut self, parent1: &Route, parent2: &Route) -> Route {
        let size = parent1.len();
        // Gera dois índices aleatórios para o crossover
        let start = self.rng.gen_range(0..size);
        let end = self.rng.gen_range(start..size);
        
        // Cria um vetor vazio para o filho com o tamanho dos pais
        // e um HashSet para verificar a presença de genes
        let mut child = Vec::with_capacity(size);
        let mut genes_in_child = HashSet::new();
        
        // Adiciona os genes do primeiro pai no intervalo selecionado
        for i in start..=end {
            let gene = parent1[i];
            child.push(gene);
            genes_in_child.insert((gene[0].to_bits(), gene[1].to_bits()));
        }
        
        // Adiciona os genes do segundo pai que não estão no filho
        let mut current_pos = 0;
        for gene in parent2 {
            if current_pos == start {
                current_pos = end + 1;
                if current_pos >= size { break; }
            }
            
            let gene_key = (gene[0].to_bits(), gene[1].to_bits());
            if !genes_in_child.contains(&gene_key) {
                child.insert(current_pos, *gene);
                genes_in_child.insert(gene_key);
                current_pos += 1;
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
        start: &Point,
        generations: usize,
        population_size: usize,
    ) -> Route {
        println!("Criando população...");
        let mut population = self.create_population(locations, start, population_size);
        println!("População criada");
        
        for generation in 0..generations {
            println!("Geração {}", generation);
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
            
            println!("Geração {} completada", generation);
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
    pub fn save_route_to_file(&self, route: &Route, file_name: &str) -> std::io::Result<()> {
        let path = Path::new(file_name);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        for point in route {
            writeln!(writer, "[{:.6}, {:.6}]", point[0], point[1])?;
        }
        
        println!("Rota salva em {}", file_name);
        Ok(())
    }
}