use anyhow::{anyhow, Result};
use ndarray::Array2;
// use ndarray::Array1;

#[derive(Debug, Clone)]
pub struct CMAESInitParams {
    pub mean: Vec<f32>,
    pub sigma: f32,
    // optional init parameters
    // pub bounds
    pub n_max_resampling: Option<i32>,
    pub seed: Option<i32>,
    pub popsize: Option<i32>,
    pub cov: Option<Array2<f32>>,
}

impl CMAESInitParams {
    pub fn validate(mut self) -> Result<Self> {
        // Default values when None supplied...
        self.n_max_resampling = Some(self.n_max_resampling.unwrap_or(100));
        self.seed = Some(self.seed.unwrap_or(16));
        let num_dims = self.mean.len() as i32;
        self.popsize = Some(
            self.popsize
                .unwrap_or_else(|| CMAESInitParams::calculate_popsize(&num_dims)),
        );
        self.cov = Some(self.cov.unwrap_or(Array2::eye(num_dims as usize)));

        // ... and validate initial parameters
        self.validate_params()
            .expect("An initial CMAES parameter is not following its constraint");
        Ok(self)
    }

    fn validate_params(&self) -> Result<()> {
        if self.sigma <= 0.0 {
            return Err(anyhow!("==> sigma must be > 0.0."));
        }
        if self.mean.len() as i32 <= 1 {
            return Err(anyhow!("==> number of dimensions must be > 1."));
        }
        if let Some(n_max_resampling) = self.n_max_resampling {
            if n_max_resampling <= 1 {
                return Err(anyhow!("==> n_max_resampling must be > 1."));
            }
        }
        if let Some(popsize) = self.popsize {
            if popsize <= 5 {
                return Err(anyhow!("==> popsize must be > 5."));
            }
        }
        if let Some(cov) = &self.cov {
            let shape = cov.shape();
            if shape[0] != self.mean.len() || shape[1] != self.mean.len() {
                return Err(anyhow!("Invalid shape of covariance matrix"));
            }
        }
        Ok(())
    }

    fn calculate_popsize(n_dim: &i32) -> i32 {
        4 + (3.0 * (*n_dim as f32).ln()).floor() as i32
    }
}
