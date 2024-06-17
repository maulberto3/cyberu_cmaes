// use ndarray::{s, Array2};
// use ndarray_rand::{rand_distr::StandardNormal, RandomExt};

use anyhow::Result;
use ndarray::{Array1, Array2};
use blas_src;
use ndarray_linalg::Eig;

use crate::{params::CmaesParams, state::CmaesState};
// use crate::state::CmaesState;

#[derive(Debug)]
pub struct Cmaes {
    pub params: CmaesParams,
    // pub state: CmaesState,
}

impl Cmaes {
    pub fn new(params: &CmaesParams) -> Result<Self> {
        // Validate initial parameters
        let params = params.clone().validate()?;
        Ok(Cmaes { params })
    }

    pub fn ask_one(self, params: &CmaesParams, state: CmaesState) -> Result<()> {
        let _ = self.eigen_decomposition(state);
        Ok(())
    }

    fn eigen_decomposition(&self, mut state: CmaesState) -> Result<()> {
        if (state.b != None) & (state.d != None) {
            (state.b, state.d);
        } else {
            // Ensure symmetric covariance
            state.cov = (&state.cov + &state.cov.t()) / 2.0;
            let (eigs, vecs) = state.cov.eig().unwrap();

            // Extract real parts of eigenvalues and eigenvectors
            let real_eigs: Array1<f32> = eigs.map(|eig| eig.re);
            let real_vecs: Array2<f32> = vecs.map(|vec| vec.re);

            println!("{:+.4?}", &real_eigs);
            println!("{:+.4?}", &real_vecs);
        }

        // D2, B = np.linalg.eigh(self._C)
        // D = np.sqrt(np.where(D2 < 0, _EPS, D2))
        // self._C = np.dot(np.dot(B, np.diag(D**2)), B.T)

        // self._B, self._D = B, D
        // return B, D

        Ok(())
    }

    // fn eigen_decomposition(&mut self) -> Result<()> {
    //     // Update eigen decomposition only once prior to first ask of pop
    //     match (self.add_params.b, self.add_params.d) {
    //         (Some(b), Some(d)) => (self.add_params.b, self.add_params.d),
    //         (_) => {
    //             if self.b. is not None and self.d is not None:
    //                 return self._B, self._D
    //             // Ensure symmetric covariance
    //             self.init_params.cov = self
    //                 .init_params
    //                 .cov
    //                 .as_ref()
    //                 .map(|cov| (cov + &cov.t()) / 2.0);
    //             println!("{:?}", self.init_params.cov);
    //         }
    //     Ok(())
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
