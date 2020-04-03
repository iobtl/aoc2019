use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_fuel(mass: u32) -> f64 {
    let mass_convert = mass as f64;

    (mass_convert / 3.0).floor() - 2.0
}
fn main() { 
    let mut fuel: Vec<f64> = Vec::new();
    // let mut file = File::open("input.txt")?;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                // Converting String to u32
                let value = ip.trim().parse::<u32>().unwrap();
                fuel.push(calculate_fuel(value));
                let sum: f64 = fuel.iter().sum();
                println!("Total fuel needed: {}", sum);

            }
        }
    }
}
