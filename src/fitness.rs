use ndarray::{Array2, Axis};
use rayon::prelude::*;
// use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn square_and_sum(pop: &Array2<f32>) -> Array2<f32> {
    let popsize = pop.shape()[0];
    // No rayon/paralell version
    // pop.map_axis(Axis(0), |row| row.mapv(|elem| elem.powi(2)).sum())
    //     .into_shape((*popsize, 1))
    //     .unwrap()
    let rows: Vec<_> = pop.axis_iter(Axis(0)).collect();
    let fitness_values: Vec<f32> = rows
        .par_iter()
        .map(|row| row.iter().map(|&elem| elem.powi(2)).sum())
        .collect();
    // Ensure output shape is (pop, fitness_value)
    Array2::from_shape_vec((popsize, 1), fitness_values).unwrap()
}

// TODO:
// Implement other objective functions
// simple std
// DEA would be graet
// Rastrigin and others
