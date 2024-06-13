// use ndarray::{s, Array2};
// use ndarray_rand::{rand_distr::StandardNormal, RandomExt};

use anyhow::Result;
use ndarray::Array1;

use crate::params::{CMAESInitParams, CMAESMoreParams};

#[derive(Debug)]
pub struct CMAES {
    pub init_params: CMAESInitParams,
    pub more_params: Option<CMAESMoreParams>,
}

impl CMAES {
    pub fn new(init_params: CMAESInitParams) -> Result<Self> {
        let init_params = init_params
            .validate()?;
        let algo = CMAES{ init_params, more_params: None }
            .gen_more_params()?;
        Ok( algo )
    }

    fn gen_more_params(mut self) -> Result<Self> {
        let b: Vec<f32> = vec![1., 2., 3.];
        self.more_params = Some(CMAESMoreParams{ b });
        // TODO: under this setting create more params
        // for now everyting needed for eigen_decomp method
        Ok(self)
    }

    // fn init_algorithm(mut self) -> Result<Self> {
    //     let algo = match self {
    //         CMAES{ init_params } => {
    //             self._B = vec![1, 2, 3, 4, 5, 6, 7, 8];
    //         },
    //         _ => unreachable!("can't reach")
    //     };
    //     Ok(algo)
    // }

    // pub fn ask_one(&self, state: &State, params: &Params) -> Result<Array1<f32>> {
    //     let (B, D) = self.eigen_decomposition();

    // }

    // fn eigen_decomposition(&self) -> Result<Array1<f32>, Array1<f32>> {
    //     if self._B and self._D is not None:
    //         return self._B, self._D


    // }

    // def ask(self) -> np.ndarray:
    // """Sample a parameter"""
    // for i in range(self._n_max_resampling):
    //     x = self._sample_solution()
    //     if self._is_feasible(x):
    //         return x
    // x = self._sample_solution()
    // x = self._repair_infeasible_params(x)
    // return x

    // def _sample_solution(self) -> np.ndarray:
    //     B, D = self._eigen_decomposition()
    //     z = self._rng.randn(self._n_dim)  # ~ N(0, I)
    //     y = cast(np.ndarray, B.dot(np.diag(D))).dot(z)  # ~ N(0, C)
    //     x = self._mean + self._sigma * y  # ~ N(m, σ^2 C)
    //     return x

    // def _eigen_decomposition(self) -> tuple[np.ndarray, np.ndarray]:
    //     if self._B is not None and self._D is not None:
    //         return self._B, self._D

    //     self._C = (self._C + self._C.T) / 2
    //     D2, B = np.linalg.eigh(self._C)
    //     D = np.sqrt(np.where(D2 < 0, _EPS, D2))
    //     self._C = np.dot(np.dot(B, np.diag(D**2)), B.T)

    //     self._B, self._D = B, D
    //     return B, D

    // def _is_feasible(self, param: np.ndarray) -> bool:
    //     if self._bounds is None:
    //         return True
    //     return cast(
    //         bool,
    //         np.all(param >= self._bounds[:, 0]) and np.all(param <= self._bounds[:, 1]),
    //     )  # Cast bool_ to bool.

    // def _repair_infeasible_params(self, param: np.ndarray) -> np.ndarray:
    //     if self._bounds is None:
    //         return param

    //     # clip with lower and upper bound.
    //     param = np.where(param < self._bounds[:, 0], self._bounds[:, 0], param)
    //     param = np.where(param > self._bounds[:, 1], self._bounds[:, 1], param)
    //     return param

    //     Array2::from_shape_vec((2, 2), vec![1., 2., 3., 4.]).unwrap()
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
