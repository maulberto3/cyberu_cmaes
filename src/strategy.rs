use anyhow::Result;

use ndarray::{s, Array1, Array2, ArrayView1, Axis};
use ndarray_rand::{rand_distr::StandardNormal, RandomExt};

use crate::{
    fitness::Fitness,
    params::{CmaesParams, CmaesParamsValid},
    state::CmaesState,
};
// use crate::state::CmaesState;

#[derive(Debug)]
pub struct Cmaes {
    pub params_valid: CmaesParamsValid,
    // pub state: CmaesState,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub x: Array1<f32>,
}

#[derive(Debug, Clone)]
pub struct Population {
    pub xs: Array2<f32>,
}

impl Cmaes {
    pub fn new(params: &CmaesParams) -> Result<Self> {
        // Instantiate Cmaes
        let params_valid = CmaesParamsValid::validate(params)?;
        Ok(Cmaes { params_valid })
    }

    fn ask_one(&self, params: &CmaesParamsValid, state: &CmaesState) -> Result<Individual> {
        // Generate one individual from params and current state
        // z ~ N(0, I)
        let z: Array1<f32> = Array1::random((params.mean.len(),), StandardNormal);

        // Rotate towards eigen i.e. y = B * D_diag * z
        let y: Array1<f32> = state.vecs.dot(&Array2::from_diag(&state.eigvs)).dot(&z);

        // Scale and translate i.e. x =  σ * y + μ
        let x: Array1<f32> = y.mapv(|elem| elem * state.sigma) + &state.mean;

        Ok(Individual { x })
    }

    pub fn ask(&self, state: &mut CmaesState) -> Result<Population> {
        // Prepare before ask population
        state.prepare_ask()?;

        // Create population
        let popsize = self.params_valid.popsize;
        let mut xs: Array2<f32> =
            Array2::zeros((popsize as usize, self.params_valid.mean.len() as usize));
        for i in 0..popsize {
            let indiv: Individual = self.ask_one(&self.params_valid, state)?;
            xs.row_mut(i as usize).assign(&indiv.x);
        }
        Ok(Population { xs })
    }

    pub fn tell(
        &self,
        mut state: CmaesState,
        pop: &mut Population,
        fitness: &mut Fitness,
    ) -> Result<CmaesState> {
        // Increment step count
        state.g += 1;

        // Sort ascending indices of fitness, i.e.e fist one is best (minimum objective)
        let mut indices: Vec<usize> = (0..fitness.fit.len()).collect();
        indices.sort_by(|&i, &j| fitness.fit[i].partial_cmp(&fitness.fit[j]).unwrap());

        // Sort population matrix
        let mut sorted_xs: Array2<f32> = Array2::zeros((pop.xs.nrows(), pop.xs.ncols()));
        for (new_idx, &original_idx) in indices.iter().enumerate() {
            sorted_xs.row_mut(new_idx).assign(&pop.xs.row(original_idx));
        }
        pop.xs = sorted_xs;

        // Sort fitness array
        let mut sorted_fit: Array1<f32> = Array1::zeros(fitness.fit.len());
        for (new_idx, &original_idx) in indices.iter().enumerate() {
            sorted_fit[new_idx] = fitness.fit[original_idx];
        }
        fitness.fit = sorted_fit;

        // Getting y back from x (see ask_one)
        pop.xs.axis_iter_mut(Axis(0)).for_each(|mut row| {
            row -= &state.mean;
            row /= state.sigma;
        });
        
        // Selection and recombination
        // Select best half of individuals
        let y_mu: Array2<f32> = pop.xs.slice(s![..self.params_valid.mu, ..]).t().to_owned();
        let weights_mu: Array1<f32> = self.params_valid.weights_prime.slice(s![..self.params_valid.mu]).to_owned();
        // Get weigthed y
        let y_w: Array2<f32> = &y_mu * &weights_mu.broadcast((y_mu.nrows(), weights_mu.len())).unwrap();
        let y_w: Array1<f32> = y_w.sum_axis(Axis(1));
        // Update mean
        state.mean = state.mean + y_w.mapv(|x| x * self.params_valid.cm * self.params_valid.sigma);

        // Step-size

        // Covariance matrix adaption

        // (eq.45)

        // (eq.46)

        // (eq.47)

        // Learning rate adaptation (enhancement)


        Ok(state)
    }

    // TODO
    // Reset required variables for next pop
    // pub fn after_tell(...) {
    // }

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
}
