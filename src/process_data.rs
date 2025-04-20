use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
type Point = [f64; 2];

pub fn get_coordinates_from_csv(file_path: &str, city_filter: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut coordinates_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 11 {  // Verifica se há dados suficientes
            continue;
        }

        let city = parts[2].trim();

        if city.eq_ignore_ascii_case(city_filter) {
            let latitude_str = format!("{}.{}", parts[7].trim(), parts[8].trim());
            let longitude_str = format!("{}.{}", parts[9].trim(), parts[10].trim());

            if let (Ok(latitude), Ok(longitude)) = (
                latitude_str.parse::<f64>(),
                longitude_str.parse::<f64>(),
            ) {
                coordinates_list.push([latitude, longitude]);
            } else {
                eprintln!("Erro ao converter coordenadas: {:?}", parts);
            }
        }
    }

    Ok(coordinates_list)
}

fn main() {
    let file_path = "pedidos_entrega.csv";
    let city = "Natal";

    match get_coordinates_from_csv(file_path, city) {
        Ok(coordinates) => {
            println!("Coordenadas encontradas para {}: {}", city, coordinates.len());
        }
        Err(e) => {
            eprintln!("Erro ao processar arquivo: {}", e);
        }
    }
}