use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct CmaesParams {
    pub mean: Vec<f32>,
    pub sigma: f32,
    // optional init parameters
    // pub bounds
    // pub n_max_resampling: Option<i32>,
    // pub seed: Option<u64>,
    pub popsize: Option<i32>,
}

impl CmaesParams {
    pub fn validate(mut self) -> Result<Self> {
        print!("Computing default values for `None` initial parameters... ");
        self.create_default_init_params()?;
        println!("Done.");

        print!("Validating initial parameters... ");
        match self.validate_init_params() {
            Ok(_) => {
                println!(" Done.")
            }
            Err(e) => {
                eprint!("An initial Cmaes parameter is not following its constraint: ");
                eprintln!("{} \n", e);
                panic!();
            }
        }
        Ok(self)
    }

    fn create_default_init_params(&mut self) -> Result<()> {
        // Impute default values when None supplied
        // self.n_max_resampling = Some(self.n_max_resampling.unwrap_or(100));
        // self.seed = Some(self.seed.unwrap_or(16));
        let num_dims = self.mean.len() as i32;
        self.popsize = Some(
            self.popsize
                .unwrap_or_else(|| CmaesParams::calculate_popsize(&num_dims).unwrap()),
        );
        Ok(())
    }

    fn validate_init_params(&self) -> Result<()> {
        // Check that the initial parameters hold their constraints
        self.check_sigma()?;
        self.check_mean_length()?;
        // self.check_n_max_resampling()?;
        self.check_popsize()?;
        Ok(())
    }

    fn check_sigma(&self) -> Result<()> {
        if self.sigma <= 0.0 {
            return Err(anyhow!("==> sigma must be > 0.0."));
        }
        Ok(())
    }

    fn check_mean_length(&self) -> Result<()> {
        if self.mean.len() <= 1 {
            return Err(anyhow!("==> number of dimensions must be > 1."));
        }
        Ok(())
    }

    // fn check_n_max_resampling(&self) -> Result<()> {
    //     if let Some(n_max_resampling) = self.n_max_resampling {
    //         if n_max_resampling <= 1 {
    //             return Err(anyhow!("==> n_max_resampling must be > 1."));
    //         }
    //     }
    //     Ok(())
    // }

    fn check_popsize(&self) -> Result<()> {
        if let Some(popsize) = self.popsize {
            if popsize <= 5 {
                return Err(anyhow!("==> popsize must be > 5."));
            }
        }
        Ok(())
    }

    fn calculate_popsize(num_dims: &i32) -> Result<i32> {
        Ok(4 + (3.0 * (*num_dims as f32).ln()).floor() as i32)
    }
}
