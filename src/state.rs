use anyhow::Result;
use ndarray::{Array1, Array2};

use crate::params::CmaesParams;

#[derive(Debug, Clone)]
pub struct CmaesState {
    pub cov: Option<Array2<f32>>,
    pub b: Option<Array1<f32>>,
    pub d: Option<Array1<f32>>,
}

impl CmaesState {
    pub fn init_state(params: &CmaesParams) -> Result<CmaesState> {
        let cov: Option<Array2<f32>> = None;
        let b: Option<Array1<f32>> = None;
        let d: Option<Array1<f32>> = None;
        Ok(CmaesState { cov, b, d })
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
