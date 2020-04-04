use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_fuel(mass: f64) -> f64 {

    let fuel = (mass / 3.0).floor() - 2.0;
    if fuel > 0.0 {
        // Passing ownership to the next iteration of the function call
        fuel + calculate_fuel(fuel) 
    } else {
        // Returning 0 if fuel is 0 or negative
        0.0
    }
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
                let value = ip.trim().parse::<f64>().unwrap();
                fuel.push(calculate_fuel(value));
                let sum: f64 = fuel.iter().sum();
                println!("Total fuel needed: {}", sum);

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel_1969() {
        assert_eq!(966.0, calculate_fuel(1969.0));
    }

    #[test]
    fn calculate_fuel_100756() {
        assert_eq!(50346.0, calculate_fuel(100756.0));
    }
}
