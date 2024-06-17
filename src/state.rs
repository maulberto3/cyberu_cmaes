use anyhow::Result;
use ndarray::{Array, Array1, Array2};
use ndarray_linalg::Eig;
use ndarray_rand::{rand_distr::StandardNormal, RandomExt};
use rand::distributions::Uniform;

use crate::params::CmaesParams;

#[derive(Debug, Clone)]
pub struct CmaesState {
    pub cov: Array2<f32>,
    pub vecs: Array2<f32>,
    pub eigvs: Array1<f32>,
}

impl CmaesState {
    pub fn init_state(params: &CmaesParams) -> Result<CmaesState> {
        // let cov: Array2<f32> = Array2::eye(params.mean.len());
        let cov: Array2<f32> = Array2::random(
            (params.mean.len(), params.mean.len()),
            Uniform::<f32>::new(-1.0, 1.0),
        );
        let vecs: Array2<f32> = Array::eye(params.mean.len());
        let eigvs: Array1<f32> = Array::from_elem((params.mean.len(),), 1.0);
        Ok(CmaesState { cov, vecs, eigvs })
    }

    pub fn prepare_ask(&mut self) -> () {
        self.eigen_decomposition();
    }

    fn eigen_decomposition(&mut self) -> () {
        // Ensure symmetric covariance
        self.cov = (&self.cov + &self.cov.t()) / 2.0;

        // Get eigenvalues and eigenvectors of covariance matrix
        let (eigvs_2, vecs) = self.cov.eig().unwrap();

        // Extract real parts of eigenvalues and eigenvectors
        let eigvs_2: Array1<f32> = eigvs_2.map(|eig| eig.re);
        let vecs: Array2<f32> = vecs.map(|vec| vec.re);

        // Convert to positive numbers (negative magnitudes dropped)
        let mut eigvs = eigvs_2.clone();
        eigvs.map_inplace(|elem| {
            if *elem < 0.0 {
                *elem = f32::EPSILON
            }
        });
        // Take sqrt of them
        eigvs.map_inplace(|elem| *elem = elem.sqrt());

        // Rotate
        self.cov = vecs
            .dot(&Array2::from_diag(&eigvs.map(|elem| elem.powi(2))))
            .dot(&vecs.t());

        self.vecs = vecs;
        self.eigvs = eigvs;
    }

    // def _sample_solution(self) -> np.ndarray:
    // B, D = self._eigen_decomposition()
    // z = self._rng.randn(self._n_dim)  # ~ N(0, I)
    // y = cast(np.ndarray, B.dot(np.diag(D))).dot(z)  # ~ N(0, C)
    // x = self._mean + self._sigma * y  # ~ N(m, σ^2 C)
    // return x
}
