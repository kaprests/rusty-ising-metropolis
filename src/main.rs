// Line tension in ising model
pub mod metropolis;
pub mod lattices;

use std::io::prelude::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::env;
use std::error::Error;

use lattices::*;
use metropolis::*;


fn main() {
    run_simulation();
    //visual_test();
}


struct Config {
    num_sweeps: usize,
    array_dim: usize,
    coupling_energy: f32,
    beta: f32,
    external_mag: f32,
}


impl Config {
    // TODO: switch to kwargs
    fn new(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        args.next();

        let num_sweeps: usize = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => {
                println!("using default num_sweeps: 100000");
                100000
            },
        };

        let array_dim: usize = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => {
                println!("using default array_dim: 32");
                32
            },
        };

        let filename = "parameters.txt";
        let params = fs::read_to_string(filename)?;
        let mut params_iter = params.split_whitespace();

        let coupling_energy:f32 = match params_iter.next() {
            Some(param) => param.parse().unwrap(),
            None => {println!("Using default coupling_energy: 1"); 1.},
        };

        let beta:f32 = match params_iter.next() {
            Some(param) => param.parse().unwrap(),
            None => {println!("Using default beta: 1"); 1.},
        };

        let external_mag:f32 = match params_iter.next() {
            Some(param) => param.parse().unwrap(),
            None => {println!("Using default external_mag: 0"); 0.},
        };

        Ok(Config{num_sweeps, array_dim, coupling_energy, beta, external_mag})
    }
}


fn run_simulation() {
    // Create config from CLI args and parameters file
    let config = Config::new(env::args()).expect("Problem with config");
    println!("PARAMETERS: ");
    println!("sweeps {}", config.num_sweeps);
    println!("arrdim {}", config.array_dim);
    println!("j {}", config.coupling_energy);
    println!("beta {}", config.beta);
    println!("ext B {}", config.external_mag);
    println!();

    // set parameters from config
    let num_sweeps = config.num_sweeps;
    let array_dim = config.array_dim;
    let coupling_energy = config.coupling_energy;
    let beta = config.beta; // T=2
    let external_mag = config.external_mag;

    // Create lattice and run simulations, and store resulting data in variable(s)
    let mut torus = Torus::new(array_dim, INF);
    let energy_array = simulate(&mut torus, num_sweeps, coupling_energy, beta.into(), external_mag);

    // Just some prints for some semi validation (I havent written tests ¯\_(ツ)_/¯)
    println!("Checks");
    println!("Final energy array: {}", energy_array[energy_array.len() - 1]);
    println!("Final energy hamiltonian: {}", torus.hamiltonian(coupling_energy, external_mag));

	// Write to file
    let  _file = File::create("./data_tmp/energies.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data_tmp/energies.txt")
        .unwrap();

    for val in energy_array.iter() {
        if let Err(e) = writeln!(file, "{}", val) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}


fn visual_test() {
    let j = 1.;
    let b = 0.;

    let mut torus = Torus::new(4, INF);
    println!("Before: ");
    torus.display();
    
    println!("energy: {}", torus.hamiltonian(j, b));
    println!("de flip: {}", torus.delta_energy_flip(2, 1, j));

    torus.flip_spin(2,1);
    torus.display();

    println!("energy: {}", torus.hamiltonian(j, b));
}


