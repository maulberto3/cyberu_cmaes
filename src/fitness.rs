use anyhow::Result;
use ndarray::{Array1, Array2, Axis};

#[derive(Debug)]
pub struct Fitness {
    pub fit: Array1<f32>,
}

pub fn square_and_sum(pop: &Array2<f32>) -> Result<Fitness> {
    // let popsize = pop.shape()[0];
    let fit = pop.map_axis(Axis(1), |row| row.mapv(|elem| elem.powi(2)).sum());
    Ok(Fitness { fit })
}

// TODO:
// Implement other objective functions
// simple std
// DEA would be graet
// Rastrigin and others
