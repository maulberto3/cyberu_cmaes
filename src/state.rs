use anyhow::Result;
use ndarray::{Array, Array1, Array2};
use ndarray_rand::RandomExt;
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
    //     fn check_covariance_matrix(&self) -> Result<()> {
    //         if let Some(cov) = &self.cov {
    //             let shape = cov.shape();
    //             if shape[0] != self.mean.len() || shape[1] != self.mean.len() {
    //                 return Err(anyhow!("Invalid shape of covariance matrix"));
    //             }
    //         }
    //         // self.cov = Some(self.cov.take().unwrap_or(Array2::eye(num_dims as usize)));

    //         Ok(())
    //     }
}
