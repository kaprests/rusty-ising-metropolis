/* 
Functions implementing the Metropolis algorithm for Ising lattices.

Three functions:

* attempt_to_flip
* lattice_sweep
* simulation

IMPROVEMENT: Use generic types to support other, potentially implemented lattices in the future
    - Also: Precalculate Gaussian weight values
*/
use ndarray::Array1;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

use crate::lattices::*;


fn attempt_to_flip(t: &mut Torus, x: usize, y: usize, r: f64, j: f32, beta: f64) -> (bool, f32) {
    let delta_energy = t.delta_energy_flip(x, y, j);

    if delta_energy <= 0. {
        return (true, delta_energy)
    } else {
        if r  < (-beta * delta_energy as f64).exp() {
            //println!("accepted increase flip");
            return (true, delta_energy)
        } else {
            //println!("discared flip");
            return (false, 0.)
        }
    }
}


fn lattice_sweep(t: &mut Torus, j: f32, beta: f64) -> f32 {
    let n: usize = t.get_n();
    let nn: usize = n*n;

    let rand_x_array = Array1::random(nn, Uniform::new(0, n));
    let rand_y_array = Array1::random(nn, Uniform::new(0, n));
    let rand_r_array = Array1::random(nn, Uniform::new(0., 1.));

    let mut delta_energy_sweep = 0.;

    for idx in 1..nn {
        let x = rand_x_array[idx] as usize;
        let y = rand_y_array[idx] as usize;
        let r = rand_r_array[idx] as f64;

        let (approved, delta_e_flip) = attempt_to_flip(t, x, y, r, j, beta);

        if approved {
            t.flip_spin(x, y)
        }

        delta_energy_sweep += delta_e_flip;
    }

    delta_energy_sweep
}


pub fn simulate(t: &mut Torus, n_sweeps: usize, j: f32, beta: f64, b: f32) -> Array1<f32> {
    let mut energy_array = Array1::zeros(n_sweeps +1);
    energy_array[0] = t.hamiltonian(j, b);

    for idx in 1..n_sweeps+1 {
        let delta_energy_sweep = lattice_sweep(t, j, beta);
        energy_array[idx] = energy_array[idx-1] + delta_energy_sweep;
    }

    energy_array
}


