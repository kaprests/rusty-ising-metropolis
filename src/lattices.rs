/*
Lattices
*/

use ndarray::{Array, Array2, Array1, arr1};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;


// Initial states
pub const ZERO: usize = 0;
pub const INF: usize = 1;


//////////////////////////////////
// Traits, common functionality //
//////////////////////////////////
pub trait Lattices {
    fn get_n(&self) -> usize {println!("Not implemented"); 1}
    fn get_spin(&self, x: usize, y: usize) -> i32 {println!("Not implemented"); 1}
    fn get_sum_nn_spins(&self, x: usize, y: usize) -> i32 {println!("Not implemented"); 1}
    fn delta_energy_flip(&self, x: usize, y: usize, j: f32) -> f32 {println!("Not implemented"); 1.}
    fn hamiltonian(&self, j: f32, b: f32) -> f32 {println!("Not implemented"); 1.}
}


///////////
// TORUS //
///////////
pub struct Torus {
    n: usize,
    matrix: Array2<i32>,
    idx_plus: Array1<usize>,
    idx_min: Array1<usize>,
}


// Torus methods
impl Torus {
    pub fn new(n: usize, initial: usize) -> Torus {
        let mut matrix = match initial {
            ZERO    => { Array2::ones((n, n)) },
            INF     => { Array2::random((n, n), Uniform::new(0, 2)) },
            _       => { panic!("Invalid initial T"); },
        };

        for spin in matrix.iter_mut() {
            if *spin == 0 {
                *spin = -1
            }
        }

        let mut idx_plus: Array1<usize> = Array1::ones(n);
        let mut idx_min: Array1<usize> = Array1::ones(n);

        for idx in 1..n {
            idx_plus[idx] = idx + 1;
            idx_min[idx] = idx - 1;
        }

        idx_plus[n-1] = 0;
        idx_min[0] = n - 1;

        Torus{n, matrix, idx_plus, idx_min}
    }


    pub fn display(&self) {
        println!("{:?}", self.matrix);
        println!("");
        println!("{:?}", self.idx_plus);
        println!("");
        println!("{:?}", self.idx_min);
    }
}


// Torus traits
impl Lattices for Torus {
    fn get_n(&self) -> usize {
        self.n
    }


    fn get_spin(&self, x: usize, y: usize) -> i32 {
        self.matrix[[y, x]]
    }


    fn get_sum_nn_spins(&self, x: usize, y: usize) -> i32 {
        let left = self.get_spin(self.idx_min[x], y);
        let right = self.get_spin(self.idx_plus[x], y);
        let up = self.get_spin(x, self.idx_min[y]);
        let down = self.get_spin(x, self.idx_plus[y]);
        
        left+right+up+down
    }


    fn delta_energy_flip(&self, x: usize, y: usize, j: f32) -> f32 {
        (2*self.get_spin(x, y) * self.get_sum_nn_spins(x, y)) as f32 * j
    }


    fn hamiltonian(&self, j: f32, b: f32) -> f32 {
        let mut energy: f32 = -b * (self.matrix.sum() as f32);

        for y in 0..self.n {
            for x in 0..self.n {
                let s = self.get_spin(x, y);
                energy += (s * self.get_sum_nn_spins(x, y)) as f32;
            }
        }

        -(j/2.) * energy
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn print() {
        println!("This is a test");
    }
}



