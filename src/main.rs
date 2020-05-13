// Line tension in ising model
pub mod metropolis;
pub mod lattices;


use std::io::prelude::*;
use std::fs::{File, OpenOptions};

use lattices::*;
use metropolis::*;


fn main() {
	run_simulation();
    //visual_test();
}



fn run_simulation() {
    let n_sweeps = 10000;
    let n = 32;
    let j = 1.;
    let beta = 1.;
    let b = 0.;

    let mut torus = Torus::new(n, INF);
    let energy_array = simulate(&mut torus, n_sweeps, j, beta, b);

    println!("Checks");
    println!("Final energy array: {}", energy_array[energy_array.len() - 1]);
    println!("Final energy hamiltonian: {}", torus.hamiltonian(j, b));

	// Write to file
    let  _file = File::create("./data/energies.txt");    
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./data/energies.txt")
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


