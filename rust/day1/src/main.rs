use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

fn calculate_fuel(mass: u32) -> f64 {
   
    let mass_f64 = mass as f64;
    let fuel =(mass_f64 / 3.0).floor();  

    fuel - 2.0
}

fn main() {
   
    let input = fs::File::open("input.txt");
    let buffered = BufReader::new(input).lines();
    
    for line in buffered.lines() {
        println!("{}", line);
    }

    let mass: u32 = 12;

    let fuel = calculate_fuel(mass);

    println!("{}", fuel);


}
