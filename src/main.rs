// Line tension in ising model
pub mod metropolis;
pub mod lattices;


use lattices::*;

use ndarray::*;


fn main() {
    // Tests of implementation requiriring human inspection
    visual_tests();
}


fn visual_tests() {
    println!("hehe");

    let torus = Torus::new(4, INF);
    torus.display();

    println!("de flip: {}", torus.delta_energy_flip(1, 1, 1.));
    println!("Energy: {}", torus.hamiltonian(1., 0.));
}



