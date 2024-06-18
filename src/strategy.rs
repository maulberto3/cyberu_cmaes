use std::io::{self, Stdout, Write};

use anyhow::Result;

use ndarray::{Array1, Array2, Axis};
use ndarray_rand::{rand_distr::StandardNormal, RandomExt};

use crate::{params::CmaesParams, state::CmaesState};
// use crate::state::CmaesState;

#[derive(Debug)]
pub struct Cmaes {
    pub params: CmaesParams,
    // pub state: CmaesState,
}

#[derive(Debug)]
pub struct Individual {
    pub x: Array1<f32>,
}

#[derive(Debug)]
pub struct Population {
    pub xs: Array2<f32>,
}

impl Cmaes {
    pub fn new(params: &CmaesParams) -> Result<Self> {
        // Instantiate Cmaes
        let params = params.clone().validate()?;
        Ok(Cmaes { params })
    }

    fn ask_one(&self, params: &CmaesParams, state: &CmaesState) -> Result<Individual> {
        // Generate one individual from params and current state
        // z ~ N(0, I)
        let z: Array1<f32> = Array1::random((params.mean.len(),), StandardNormal);

        // Rotate towards eigen i.e. y = B * D_diag * z
        let y = &state.vecs.dot(&Array2::from_diag(&state.eigvs)).dot(&z);

        // Scale and translate i.e. x = μ + σ * y
        let x = &state.mean + y.map(|elem| elem * state.sigma);

        Ok(Individual { x })
    }

    pub fn ask(&self, params: &CmaesParams, state: &CmaesState) -> Result<Population> {
        let popsize = self.params.popsize.unwrap();

        let mut xs: Array2<f32> =
            Array2::zeros((popsize as usize, self.params.mean.len() as usize));

        for i in 0..popsize {
            let indiv: Individual = self.ask_one(params, state)?;
            xs.row_mut(i as usize).assign(&indiv.x); // Assign individual to population matrix
        }

        Ok(Population { xs })
    }

    // TODO
    // Whether the draw individual is within bounds supplied
    // If not, re-draw given self._n_max_resampling, or
    // ultimately, just clip values within bounds
    // fn _is_feasible()
    // fn _repair_infeasible_params

    // TODO
    // As per repo example:
    // ```
    // def main():
    //     optimizer = CMA(mean=np.zeros(2), sigma=1.3)
    //     print(" g    f(x1,x2)     x1      x2  ")
    //     print("===  ==========  ======  ======")
    //     while True:
    //         solutions = []
    //         for _ in range(optimizer.population_size):
    //             x = optimizer.ask()
    //             value = quadratic(x[0], x[1])
    //             solutions.append((x, value))
    //             print(
    //                 f"{optimizer.generation:3d}  {value:10.5f}"
    //                 f"  {x[0]:6.2f}  {x[1]:6.2f}"
    //             )
    //         optimizer.tell(solutions)
    //         if optimizer.should_stop():
    //             break
    // ```
    // TODO: make one go for population, no loop
    // as suggested in repo example, attached above
    // If ask_one is independent, try to paralellize

    // pub fn tell(&self, params: &CmaesParams, state: &CmaesState, pop: &Population, fitness: &fit)

    // TODO
    // fn ask() -> ...

    // TODO
    // Adjust given fitness values
    // pub fn tell(&self, &params, &mut state, indiv: Array2<f32>, fitness: Array2<f32>) -> Result<()> {

    //     Ok(())
    // }

    // TODO
    // Reset required variables for next pop
    // pub fn after_tell(...) {
    // }
}
