use ndarray::{Array1, Array2, Axis};

pub fn square_and_sum(pop: &Array2<f32>) -> Array1<f32> {
    // let popsize = pop.shape()[0];
    pop.map_axis(Axis(1), |row| row.mapv(|elem| elem.powi(2)).sum())
        // .into_shape((*popsize, 1))
        // .unwrap()
}

// TODO:
// Implement other objective functions
// simple std
// DEA would be graet
// Rastrigin and others
