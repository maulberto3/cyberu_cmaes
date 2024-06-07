use ndarray::{s, Array2};
use ndarray_rand::{rand_distr::StandardNormal, RandomExt};

use crate::{
    params::{CMAESParams, Params},
    states::{CMAESState, State},
};

#[derive(Debug)]
pub enum Algo {
    CMAES(usize, usize),
    // OtherAlgo,
}

impl Algo {
    pub fn default_params(&self) -> Params {
        match self {
            Algo::CMAES(_, _) => Params::CMAES(CMAESParams::default_params()),
            _ => Params::OtherParams,
        }
    }

    pub fn init_algorithm(&self, params: &Params) -> State {
        match (self, params) {
            (Algo::CMAES(_, num_dims), Params::CMAES(_)) => {
                State::CMAES(CMAESState::init_state(num_dims, params))
            }
            _ => State::OtherState,
        }
    }

    // pub fn ask(&self, state: &State, params: &Params) -> Array2<f32> {

    // }

    // pub fn tell(
    //     &self,
    //     pop: Array2<f32>,
    //     fitness: Array2<f32>,
    //     state: State,
    //     params: &Params,
    // ) -> State {
    //     }
}
