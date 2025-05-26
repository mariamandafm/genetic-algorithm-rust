use std::error::Error;
use std::thread;
use crate::genetic_algorithm::{genetic_algorithm, total_distance, save_distance_to_file, Route};
type Point = [f64; 2];
use rand::{Rng, thread_rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::Instant;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex, mpsc};

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

pub fn calculate_best_route_v2(file_path: &str) {
    let file_path = file_path.to_string();

    let num_threads = 12;
    let (sender, receiver) = mpsc::channel::<Option<Vec<[f64; 2]>>>();
    let file_lock = Arc::new(Mutex::new(()));

    let reader_sender = sender.clone();

    let reader = thread::spawn(move || {
        let file = File::open(file_path).expect("Erro ao abrir arquivo");
        let reader = BufReader::new(file);
        let mut current_group: Vec<[f64; 2]> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            
            if line == "-"{
                if !current_group.is_empty() {
                    //println!("{}, coordenadas {} {}", current_group.len(), current_group[0][0], current_group[0][1]);
                    reader_sender.send(Some(current_group.clone())).unwrap();
                    current_group.clear();
                }
                continue;
            }

            let parts: Vec<&str> = line.split(';').collect();
            
            if parts.len() != 2 {
                continue;
            }
            if let (Ok(latitude), Ok(longitude)) = (
                parts[0].replace(",", ".").parse::<f64>(),
                parts[1].replace(",", ".").parse::<f64>(),
            ) {
                current_group.push([latitude, longitude]);
            } else {
                //eprintln!("Erro ao converter coordenadas: {:?}", parts);
            }
            // let test_group = generate_route(50_000);
        }
        for _ in 0..num_threads {
            reader_sender.send(None).unwrap();
        }
    });

    let counter = Arc::new(Mutex::new(0));
    let receiver = Arc::new(Mutex::new(receiver));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let receiver = Arc::clone(&receiver);
        let counter = Arc::clone(&counter);
        let file_lock = Arc::clone(&file_lock);

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let data = {
                    let lock = receiver.lock().unwrap();
                    lock.recv().unwrap()
                };

                match data {
                    Some(group) => {
                        let best_route = genetic_algorithm(&mut rng, &group, 10, 10);
                        let distance = total_distance(&best_route);

                        let group_number = {
                            let mut count = counter.lock().unwrap();
                            let id = *count;
                            *count += 1;
                            id
                        };

                        println!("Grupo {}: Distância total: {}", group_number, distance);

                        // let _ = match save_distance_to_file(distance, group_number) {
                        //     Ok(_) => (),
                        //     Err(e) => eprintln!("Erro ao salvar arquivo: {}", e),
                        // };
                    }
                    None => break,
                }
            }
        });

        handles.push(handle);
    }

    reader.join().unwrap();
    for h in handles {
        h.join().unwrap();
    }
}
