/* 
Binary crate with functions implementing the Metropolis algorithm for Ising lattices.

Three functions:

* attempt_to_flip
* lattice_sweep
* simulation

*/
use crate::lattices::*;


fn attempt_to_flip(t: Torus, x: usize, y: usize, r: f32, j: f32, beta: f64) -> (bool, f32) {
    let delta_energy = t.delta_energy_flip(x, y, j);

    if delta_energy <= 0 {
        return true, delta_energy
    } else {
        if r < (beta * delta_energy as f64).exp() {
            return true, delta_energy
        } else {
            return false, 0
        }
    }
}


fn lattice_sweep() {
    println!("private");
}


pub fn simulate(torus: Torus) {
    println!("{}", torus.get_n());
    println!("public");
}


